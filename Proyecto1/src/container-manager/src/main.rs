use ctrlc;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

// Obtener el timestamp actual en formato RFC3339
fn get_current_timestamp() -> String {
    chrono::Utc::now().to_rfc3339()
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

    // Filtrar el proceso containerd-shim que quieres ignorar
    let filtered_processes: Vec<Process> = system_info
        .processes
        .into_iter()
        .filter(|p| {
            !(p.pid == 6658
                && p.name == "containerd-shim"
                && p.cmd_line
                    .contains("f0629ea40571ef3dc5b576afb67f91acc0c0bd4268d806df886fcc22be520bfb"))
        })
        .collect();

    // Guardamos los procesos en una lista
    let mut processes_list: Vec<Process> = filtered_processes;

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
    let mut remaining_processes: Vec<Process> = Vec::new();

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
        remaining_processes.push(process.clone());
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
        remaining_processes.push(process.clone());
    }

    /* ------------------------- ELIMINACION DE PROCESOS ------------------------ */

    let mut handles = vec![];
    for process in middle {
        let log_process = LogProcess {
            pid: process.pid,
            container_id: process.get_container_id().to_string(),
            name: process.name.clone(),
            memory_usage: process.memory_usage,
            cpu_usage: process.cpu_usage,
        };

        log_proc_list.push(log_process.clone());

        let container_id = process.get_container_id().to_string();
        let handle = thread::spawn(move || {
            let _output = kill_container(&container_id);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread failed");
    }

    /* ---------------------------------- POST ---------------------------------- */

    let mut json_data: HashMap<String, serde_json::Value> = HashMap::new();
    json_data.insert(
        "total_ram".to_string(),
        serde_json::json!(system_info.ram_total),
    );
    json_data.insert(
        "used_ram".to_string(),
        serde_json::json!(system_info.ram_used),
    );
    json_data.insert(
        "free_ram".to_string(),
        serde_json::json!(system_info.ram_available),
    );
    json_data.insert(
        "timestamp".to_string(),
        serde_json::json!(get_current_timestamp()),
    );

    let mut process_list = Vec::new();
    for process in remaining_processes {
        process_list.push(serde_json::json!({
            "pid": process.pid,
            "name": process.name,
            "vsz": process.vsz,
            "rss": process.rss,
            "memory_usage": process.memory_usage,
            "cpu_usage": process.cpu_usage,
            "cmdline": process.cmd_line
        }));
    }
    json_data.insert("processes".to_string(), serde_json::json!(process_list));

    // Hacer el POST
    let client = Client::new();
    let res = client
        .post("http://0.0.0.0:8000/logs")
        .json(&json_data)
        .send();

    match res {
        Ok(response) => println!(">>>>>>>>>>>> POST request successful: {:?}", response),
        Err(e) => println!(">>>>>>>>>>>> POST request failed: {:?}", e),
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
    println!("----------------------------------- FIN ----------------------------------");
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
    // Ejecutar el archivo docker-compose.yaml
    println!("Iniciando servicios de Docker Compose...");
    let docker_compose_path = "/home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/container-manager/logs-manager/docker-compose.yaml";
    let mut docker_compose = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(docker_compose_path)
        .arg("up")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start docker-compose");

    // Bandera para controlar el ciclo del bucle
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    // Ejecutar el script al iniciar el servicio
    let mut script_process: Child = Command::new("/bin/bash")
        .arg(
            "/home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/3-cronjob.sh",
        )
        .spawn()
        .expect("Failed to start script");

    // Manejar la señal SIGINT (Ctrl+C)
    ctrlc::set_handler(move || {
        println!("Ctrl+C Detectado!!!!!");
        running_clone.store(false, Ordering::Relaxed);

        // Crear un cliente HTTP para hacer las solicitudes GET
        let client = Client::new();

        // Detener el script al finalizar el servicio
        println!("Deteniendo el cronjob...");
        script_process.kill().expect("Failed to stop script");

        // Hacer GET a /graph
        match client.get("http://0.0.0.0:8000/graph").send() {
            Ok(response) => println!("GET /graph response: {:?}", response),
            Err(e) => println!("GET /graph request failed: {:?}", e),
        }

        // Detener el servicio de rust
        println!("Deteniendo el servicio de rust");
        process::exit(0); // Finaliza el programa
    })
    .expect("Error setting Ctrl-C handler");

    // Procesar información mientras el flag `running` es verdadero
    while running.load(Ordering::Relaxed) {
        match docker_compose.try_wait() {
            Ok(Some(status)) => {
                println!("docker-compose exited with: {}", status);
                // Reiniciar si docker-compose se detuvo inesperadamente
                docker_compose = Command::new("docker")
                    .arg("compose")
                    .arg("-f")
                    .arg(docker_compose_path)
                    .arg("up")
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Failed to restart docker-compose");
            }
            Ok(None) => {
                // docker-compose sigue corriendo
            }
            Err(e) => {
                println!("Failed to check docker-compose status: {}", e);
            }
        }
        // Esperar 10 segundos
        thread::sleep(Duration::from_secs(10));

        let system_info: Result<SystemInfo, _>;

        let json_str = read_proc_file("sysinfo_201712602").unwrap();

        system_info = parse_proc_to_struct(&json_str);

        match system_info {
            Ok(info) => {
                analyzer(info);
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }
    }
}
