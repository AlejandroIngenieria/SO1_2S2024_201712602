use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/* -------------------------------------------------------------------------- */
/*                                 ESTRUCTURAS                                */
/* -------------------------------------------------------------------------- */
// Informacion de memoria
#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "RAM total")]
    ram_total: u64,
    #[serde(rename = "RAM utilizada")]
    ram_used: u64,
    #[serde(rename = "RAM disponible")]
    ram_available: u64,
    #[serde(rename = "Processes")]
    processes: Vec<Process>,
}

// Informacion de procesos relacionados con Docker
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "Vsz")]
    vsz: u64,
    #[serde(rename = "Rss")]
    rss: u64,
    #[serde(rename = "Memory_Usage")]
    memory_usage: f64,
    #[serde(rename = "CPU_Usage")]
    cpu_usage: f64,
}

// Informacion a mostrar
#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    memory_usage: f64,
    cpu_usage: f64,
}

/* -------------------------------------------------------------------------- */
/*                                   METODOS                                  */
/* -------------------------------------------------------------------------- */

// Funcion para llenar el campo cmdline
impl Process {
    fn get_container_id(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}

/* -------------------------------------------------------------------------- */
/*                                   TRAITS                                   */
/* -------------------------------------------------------------------------- */

// Necesario para poder comparar
impl Eq for Process {}

// Necesario para poder comparar en todos los aspectos posibles
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cpu_usage
            .partial_cmp(&other.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                self.memory_usage
                    .partial_cmp(&other.memory_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

// Necesario para manejar casos invalidos
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/* -------------------------------------------------------------------------- */
/*                                  FUNCIONES                                 */
/* -------------------------------------------------------------------------- */

// Mata el contenedor de Docker por el id
fn kill_container(id: &str) -> std::process::Output {
    let output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Eliminando el contenedor con id: {}", id);

    output
}

/* -------------------------------------------------------------------------- */
/*                                 ANALIZADOR                                 */
/* -------------------------------------------------------------------------- */

fn analyzer(system_info: SystemInfo) {
    println!("--------------------------------------------------------------------------");
    println!("                          INFORMACION DE MEMORIA                          ");
    println!("--------------------------------------------------------------------------\n\n");

    println!("* RAM total: {} KB", system_info.ram_total);
    println!("* RAM utilizada: {} KB", system_info.ram_used);
    println!("* RAM disponible: {} KB", system_info.ram_available);

    println!("\n\n--------------------------------------------------------------------------");
    println!("                     PROCESOS RELACIONADOS CON DOCKER                     ");
    println!("--------------------------------------------------------------------------\n\n");

    // Guardamos los procesos en una lista
    let mut processes_list: Vec<Process> = system_info.processes;

    // 1. Ordenamos primero por uso de CPU (de mayor a menor)
    processes_list.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // 2. Después ordenamos por uso de RAM (de mayor a menor)
    processes_list.sort_by(|a, b| {
        b.memory_usage
            .partial_cmp(&a.memory_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // 3. Luego por VSZ (de mayor a menor)
    processes_list.sort_by(|a, b| b.vsz.cmp(&a.vsz));

    // 4. Finalmente por RSS (de mayor a menor)
    processes_list.sort_by(|a, b| b.rss.cmp(&a.rss));

    // Mantener los 2 primeros como alto consumo y los últimos 3 como bajo consumo
    let mut log_proc_list: Vec<LogProcess> = Vec::new();

    // Verificamos que hay al menos 5 procesos (2 de alto y 3 de bajo)
    if processes_list.len() < 5 {
        println!("No hay suficientes procesos para aplicar las restricciones.");
        return;
    }

    // Los 2 primeros contenedores deben ser de alto consumo
    let (high_consumption, rest) = processes_list.split_at(2);

    // Los últimos 3 contenedores deben ser de bajo consumo
    let (middle, low_consumption) = rest.split_at(rest.len().saturating_sub(3));

    println!("------------------------------ ALTO CONSUMO ------------------------------\n");
    for process in high_consumption {
        println!(
            "* PID: {}\n* Name: {}\n* Container ID: {}\n* CPU Usage: {}\n* Memory Usage: {}\n\n",
            process.pid,
            process.name,
            process.get_container_id(),
            process.cpu_usage,
            process.memory_usage
        );
    }

    println!("------------------------------ BAJO CONSUMO ------------------------------\n");
    for process in low_consumption {
        println!(
            "* PID: {}\n* Name: {}\n* Container ID: {}\n* CPU Usage: {}\n* Memory Usage: {}\n\n",
            process.pid,
            process.name,
            process.get_container_id(),
            process.cpu_usage,
            process.memory_usage
        );
    }

    // Si hay más procesos en la lista intermedia, debemos eliminarlos
    for process in middle {
        let log_process = LogProcess {
            pid: process.pid,
            container_id: process.get_container_id().to_string(),
            name: process.name.clone(),
            memory_usage: process.memory_usage,
            cpu_usage: process.cpu_usage,
        };

        log_proc_list.push(log_process.clone());

        // Matamos el contenedor
        let _output = kill_container(&process.get_container_id());
    }

    println!("------------------------- CONTENEDORES ELIMINADOS ------------------------\n");
    for process in log_proc_list {
        println!(
            "PID: {}, Name: {}, Container ID: {}, CPU Usage: {}, Memory Usage: {}",
            process.pid,
            process.name,
            process.container_id,
            process.cpu_usage,
            process.memory_usage
        );
    }

    println!("--------------------------------------------------------------------------");
    println!("--------------------------------------------------------------------------");
    println!("--------------------------------------------------------------------------");
}

// Funcion para leer la informacion del proceso
fn read_proc_file(file_name: &str) -> io::Result<String> {
    let path = Path::new("/proc").join(file_name);
    let mut file = File::open(path)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

// Deserializamos el contenido del archivo del proceso en /proc/ a un vector de procesos
fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

    Ok(system_info)
}

fn main() {
    loop {
        let system_info: Result<SystemInfo, _>;

        let json_str = read_proc_file("sysinfo_201712602").unwrap();

        system_info = parse_proc_to_struct(&json_str);

        match system_info {
            Ok(info) => {
                analyzer(info);
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
