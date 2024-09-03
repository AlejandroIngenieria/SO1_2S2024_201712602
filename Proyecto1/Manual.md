### UNIVERSIDAD DE SAN CARLOS DE GUATEMALA

### FACULTAD DE INGENIERÍA

### ESCUELA DE CIENCIAS Y SISTEMAS

#### SISTEMAS OPERATIVOS 1 SECCIÓN N

**ING. JESÚS GUZMÁN POLANCO**  
**AUX. ALVARO NORBERTO GARCÍA**  
**AUX. SERGIO ALFONSO FERRER GARCÍA**

**SEGUNDO SEMESTRE 2024**

---

## PROYECTO 1: Gestor de Contenedores

**Nombre:** Josue Alejandro Pérez Benito  
**Carné:** 201712602

**Fecha de Entrega:** 08 de septiembre del 2024

---
# Manual tecnico
1. [**Script Creador de Contenedores**](#1-script-creador-de-contenedores)
2. [**Módulo de Kernel**](#2-módulo-de-kernel)
3. [**Servicio de Rust**](#3-servicio-de-rust)
4. [**Administrador de logs**](#4-administrador-de-logs)

---

<!-- ----------------------------------------------------------------------- -->
<!--                     SCRIPT CREADOR DE CONTENEDORES                      -->
<!-- ----------------------------------------------------------------------- -->
# 1. Script Creador de Contenedores
## Creacion de las 4 imagenes base
### Alto consumo de RAM
#### Archivo
```Python
# Este script consume mucha RAM creando una lista enorme
import time

# Crear una lista muy grande en memoria
big_list = []

# Rellenar la lista con cadenas de texto grandes
for i in range(1000000):
    # Cada elemento es una cadena de 1 millón de caracteres
    big_list.append("X" * 1000000)
    time.sleep(0.1)  # Agregar un pequeño retraso entre las inserciones

# Mantener el programa en ejecución para que puedas observar el uso de RAM
time.sleep(3600)  # Espera una hora

```
#### Dockerfile
```Dockerfile
# Dockerfile para alto consumo de RAM
FROM python:3.9-slim

# Crear un script de Python que consuma bastante memoria
COPY /Docker/alto_ram.py /Docker/alto_ram.py

# Ejecutar el script y mantener el contenedor corriendo
CMD ["python", "/Docker/alto_ram.py"]
```
### Alto consumo de CPU
#### Archivo
```Python
# Este script consume mucha CPU realizando cálculos intensivos
import time
import math
for i in range(1000000):
    math.sqrt(12345)
    time.sleep(0.1)
```
#### Dockerfile
```Dockerfile
# Dockerfile para alto consumo de CPU
FROM python:3.9-slim

# Copiar el script de Python al contenedor
COPY /Docker/alto_cpu.py /Docker/alto_cpu.py

# Ejecutar el script y mantener el contenedor corriendo
CMD ["python", "/Docker/alto_cpu.py"]
```
### Bajo consumo de RAM
#### Archivo
```Python
import time

# Este script realiza cálculos simples de manera continua


def simple_calculations():
    result = 0
    for i in range(100000):
        result += i * i  # Realiza una operación matemática simple
    return result


# Ejecuta los cálculos en un bucle
while True:
    result = simple_calculations()
    print(f"Resultado: {result}")
    time.sleep(1)  # Espera 1 segundo antes de realizar la siguiente iteración

```
#### Dockerfile
```Dockerfile
# Dockerfile para bajo consumo de RAM
FROM python:3.9-slim

# Crear un script de Python que no consuma mucha RAM
COPY /Docker/bajo_ram.py /Docker/bajo_ram.py

# Ejecutar el script y mantener el contenedor corriendo
CMD ["python", "/Docker/bajo_ram.py"]
```
### Bajo consumo de CPU
#### Archivo
```Python
# Script de bajo consumo usando Flask
from flask import Flask

app = Flask(__name__)


@app.route('/')
def hello_world():
    return 'Hello, World!'


if __name__ == '__main__':
    app.run(host='0.0.0.0')

```
#### Dockerfile
```Dockerfile
# Dockerfile para bajo consumo de CPU
FROM python:3.9-slim

# Crear un script de Python que no consuma mucha CPU
COPY /Docker/bajo_cpu.py /Docker/bajo_cpu.py

RUN pip install flask
# Ejecutar el script y mantener el contenedor corriendo
CMD ["python", "/Docker/bajo_cpu.py"]

```
### Script para automatizar la tarea
```bash
#!/bin/bash

# Nombres de las imágenes
IMAGENES=("alto_cpu" "alto_ram" "bajo_cpu" "bajo_ram")

# Rutas a los Dockerfiles
DOCKERFILES=("Docker/Dockerfile1" "Docker/Dockerfile2" "Docker/Dockerfile3" "Docker/Dockerfile4")

# Crear las imágenes
for i in ${!IMAGENES[@]}; do
    echo "Construyendo la imagen ${IMAGENES[$i]} usando ${DOCKERFILES[$i]}..."
    docker build -t ${IMAGENES[$i]} -f ${DOCKERFILES[$i]} .
    if [ $? -eq 0 ]; then
        echo "Imagen ${IMAGENES[$i]} creada exitosamente."
    else
        echo "Error al crear la imagen ${IMAGENES[$i]}."
        exit 1
    fi
done

```
## Creacion de los contenedores
### Script para realizar dicha tarea
```bash
#!/bin/bash

# Nombres de las imágenes Docker
IMAGENES=("alto_cpu" "alto_ram" "bajo_cpu" "bajo_ram")

# Número de contenedores a crear
NUM_CONTENEDORES=10

# Función para generar un nombre aleatorio para el contenedor
generar_nombre_contenedor() {
    cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1
}

# Crear contenedores
crear_contenedores() {
    for i in $(seq 1 $NUM_CONTENEDORES); do
        # Seleccionar una imagen aleatoria
        IMAGEN=${IMAGENES[$RANDOM % ${#IMAGENES[@]}]}
        
        # Generar un nombre aleatorio para el contenedor
        NOMBRE_CONTAIN=$(generar_nombre_contenedor)
        
        # Crear el contenedor
        echo "Creando contenedor ${NOMBRE_CONTAIN} usando la imagen ${IMAGEN}..."  >> /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/log_ejecucion.log
        /usr/bin/docker run -d --name ${NOMBRE_CONTAIN} ${IMAGEN} >> /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/log_ejecucion.log 2>&1
        
        if [ $? -eq 0 ]; then
            echo "Contenedor ${NOMBRE_CONTAIN} creado exitosamente." >> /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/log_ejecucion.log
        else
            echo "Error al crear el contenedor ${NOMBRE_CONTAIN}." >> /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/log_ejecucion.log
        fi
    done
}

# Ejecutar la función para crear contenedores
crear_contenedores

```

### Logs del script
![alt text](<imgs/3. script.png>)

### Comando para detener contenedores creados por el script
```bash
docker stop $(docker ps --filter "ancestor=bajo_ram" -q) $(docker ps --filter "ancestor=alto_ram" -q) $(docker ps --filter "ancestor=alto_cpu" -q) $(docker ps --filter "ancestor=bajo_cpu" -q)
```
### Comando para eliminar contenedores creados por el script
```bash
docker rm $(docker ps -a --filter "ancestor=bajo_ram" -q) $(docker ps -a --filter "ancestor=alto_ram" -q) $(docker ps -a --filter "ancestor=alto_cpu" -q) $(docker ps -a --filter "ancestor=bajo_cpu" -q)
```
### Comando para eliminar todos los contenedores en docker
```bash
docker rm -f $(docker ps -a -q)
```
### Comando para eliminar todas las imagenes de docker
```bash
docker rmi -f $(docker images -q)
```
## Configurar Cron Job 
### Opcion 1 (Cada 30 segundos) -> usando crontab en bash
```bash
crontab -e
```
#### Añadimos la configuracion para 30 segundos con la **(ruta donde este el archivo que crea los contenedores)**
```bash                                                                       
* * * * * /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/2-createContainers.sh
* * * * * sleep 30; /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/2-createContainers.sh
```
### Opcion 2 (cada 25 segundos) -> Gracias a los hilos en Rust
```bash
#!/bin/bash

while true; do
  /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/2-createContainers.sh
  sleep 25
done
```

#### Verificando la creacion de los contenedores
![alt text](<imgs/4. containers.png>)

# 2. Módulo de Kernel
## Creacion del modulo en C
```c
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/string.h>
#include <linux/init.h>
#include <linux/proc_fs.h>
#include <linux/seq_file.h>
#include <linux/mm.h>
#include <linux/sched.h>
#include <linux/timer.h>
#include <linux/jiffies.h>
#include <linux/uaccess.h>
#include <linux/tty.h>
#include <linux/sched/signal.h>
#include <linux/fs.h>
#include <linux/slab.h>
#include <linux/sched/mm.h>
#include <linux/binfmts.h>
#include <linux/timekeeping.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Josue Alejandro Perez Benito");
MODULE_DESCRIPTION("Modulo para leer informacion de memoria y procesos de docker");
MODULE_VERSION("1.0");

#define PROC_NAME "sysinfo_201712602"
#define MAX_CMDLINE_LENGTH 256
#define CONTAINER_ID_LENGTH 64

// Función para obtener la línea de comandos de un proceso
static char *get_process_cmdline(struct task_struct *task)
{
    struct mm_struct *mm;
    char *cmdline, *p;
    unsigned long arg_start, arg_end, env_start;
    int i, len;

    cmdline = kmalloc(MAX_CMDLINE_LENGTH, GFP_KERNEL);
    if (!cmdline)
        return NULL;

    mm = get_task_mm(task);
    if (!mm)
    {
        kfree(cmdline);
        return NULL;
    }

    down_read(&mm->mmap_lock);
    arg_start = mm->arg_start;
    arg_end = mm->arg_end;
    env_start = mm->env_start;
    up_read(&mm->mmap_lock);

    len = arg_end - arg_start;
    if (len > MAX_CMDLINE_LENGTH - 1)
        len = MAX_CMDLINE_LENGTH - 1;

    if (access_process_vm(task, arg_start, cmdline, len, 0) != len)
    {
        mmput(mm);
        kfree(cmdline);
        return NULL;
    }

    cmdline[len] = '\0';

    p = cmdline;
    for (i = 0; i < len; i++)
        if (p[i] == '\0')
            p[i] = ' ';

    mmput(mm);
    return cmdline;
}

// Función para mostrar la información en el archivo /proc en formato JSON
static int sysinfo_show(struct seq_file *m, void *v)
{
    struct sysinfo si;
    struct task_struct *task;
    unsigned long total_jiffies = jiffies;
    int first_process = 1;

    // Obtenemos la información de memoria
    si_meminfo(&si);
    unsigned long totalram = si.totalram * 4;
    unsigned long freeram = si.freeram * 4;
    unsigned long usedram = totalram - freeram;

    seq_printf(m, "{\n");
    seq_printf(m, "\"RAM total\": %lu,\n", totalram);
    seq_printf(m, "\"RAM utilizada\": %lu,\n", usedram);
    seq_printf(m, "\"RAM disponible\": %lu,\n", freeram);
    seq_printf(m, "\"Docker\": [\n");

    // Iteramos sobre los procesos
    for_each_process(task)
    {
        if (strcmp(task->comm, "containerd-shim") == 0)
        {
            unsigned long vsz = 0;
            unsigned long rss = 0;
            unsigned long mem_usage = 0;
            unsigned long cpu_usage = 0;
            char *cmdline = NULL;

            if (task->mm)
            {
                vsz = task->mm->total_vm << (PAGE_SHIFT - 10);
                rss = get_mm_rss(task->mm) << (PAGE_SHIFT - 10);
                mem_usage = (rss * 10000) / totalram;
            }

            unsigned long total_time = task->utime + task->stime;
            cpu_usage = (total_time * 10000) / total_jiffies;
            cmdline = get_process_cmdline(task);

            if (!first_process)
            {
                seq_printf(m, ",\n");
            }
            else
            {
                first_process = 0;
            }

            seq_printf(m, "  {\n");
            seq_printf(m, "    \"PID\": %d,\n", task->pid);
            seq_printf(m, "    \"Name\": \"%s\",\n", task->comm);
            seq_printf(m, "    \"Cmdline\": \"%s\",\n", cmdline ? cmdline : "N/A");
            seq_printf(m, "    \"Vsz\": %lu,\n", vsz);
            seq_printf(m, "    \"Rss\": %lu,\n", rss);
            seq_printf(m, "    \"MemoryUsage\": %lu.%02lu,\n", mem_usage / 100, mem_usage % 100);
            seq_printf(m, "    \"CPUUsage\": %lu.%02lu\n", cpu_usage / 100, cpu_usage % 100);
            seq_printf(m, "  }");

            if (cmdline)
            {
                kfree(cmdline);
            }
        }
    }

    seq_printf(m, "\n]\n}\n");
    return 0;
}

// Función que se ejecuta al abrir el archivo /proc
static int sysinfo_open(struct inode *inode, struct file *file)
{
    return single_open(file, sysinfo_show, NULL);
}

// Estructura que contiene las operaciones del archivo /proc
static const struct proc_ops sysinfo_ops = {
    .proc_open = sysinfo_open,
    .proc_read = seq_read,
};

// Función de inicialización del módulo
static int __init sysinfo_init(void)
{
    proc_create(PROC_NAME, 0, NULL, &sysinfo_ops);
    printk(KERN_INFO "sysinfo_json modulo cargado\n");
    return 0;
}

// Función de limpieza del módulo
static void __exit sysinfo_exit(void)
{
    remove_proc_entry(PROC_NAME, NULL);
    printk(KERN_INFO "sysinfo_json modulo desinstalado\n");
}

module_init(sysinfo_init);
module_exit(sysinfo_exit);

```

## Makefile
```Makefile
obj-m += sysinfo.o

all:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean
```
## Instalacion del modulo de kernel
En el mismo directorio del archivo makefile ejecutamos:
```bash
make
```
Al haber ejecutado el comando se nos habran creado todos los archivos necesarios para la instalacion del modulo.
Usamos el archivo *nombre_modulo.ko*

```bash
sudo insmod nombre_modulo.ko
```

Verificamos la ejecucion del modulo con CAT

```bash
cat /proc/nombre_del_modulo
```

Para eliminar el modulo usamos el siguiente comando:
```bash
sudo rmmod nombre_del_modulo
```

# 3. Servicio de Rust

Configuraciones del entorno en Rust

```rust
[package]
name = "container-manager"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.12.6", features = ["json","blocking"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
ctrlc = "3.2"
```

Servicio en rust
```rust
use ctrlc;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;
use std::process::{Child, Command};
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
        .post("http://127.0.0.1:8000/logs")
        .json(&json_data)
        .send();

    match res {
        Ok(response) => println!("POST request successful: {:?}", response),
        Err(e) => println!("POST request failed: {:?}", e),
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
        match client.get("http://127.0.0.1:8000/graph").send() {
            Ok(response) => println!("GET /graph response: {:?}", response),
            Err(e) => println!("GET /graph request failed: {:?}", e),
        }

        // Hacer GET a /view
        match client.get("http://127.0.0.1:8000/view").send() {
            Ok(response) => println!("GET /view response: {:?}", response),
            Err(e) => println!("GET /view request failed: {:?}", e),
        }

        // Detener el servicio de rust
        println!("Deteniendo el servicio de rust");
        process::exit(0); // Finaliza el programa
    })
    .expect("Error setting Ctrl-C handler");

    // Procesar información mientras el flag `running` es verdadero
    while running.load(Ordering::Relaxed) {
        
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

```

# 4. Administrador de logs

## API
Para el desarrollo del administrador de los logs del proyecto usamos un entorno virtual con python.
```bash
python3 -m venv env         # creamos el entorno virtual
source env/bin/activate     # activamos el entorno virtual
```

Para el desarrollo de nuestro adminstrador de logs usamos [fastapi]

[fastapi]:https://fastapi.tiangolo.com/

```bash
pip install "fastapi[standard]"
```

Usamos herramientas como mathplotlib para graficar
```bash
pip install mathplotlib
```

Para la informacion manejamos el siguiente modelo
```python
class LogProcess(BaseModel):
    pid: int
    name: str
    vsz: int
    rss: int
    memory_usage: float
    cpu_usage: float
    cmdline: str


class LogSystem(BaseModel):
    total_ram: int
    used_ram: int
    free_ram: int
    timestamp: str
    processes: List[LogProcess]

```

Nuestra API es la siguiente:
```python
from fastapi import FastAPI                     #type:ignore
from fastapi.responses import HTMLResponse      # type:ignore
from fastapi.staticfiles import StaticFiles     # type:ignore
import os
import json
from typing import List
from models.models import LogSystem
import matplotlib.pyplot as plt #type:ignore

app = FastAPI()


@app.get("/")
def read_root():
    return {"Logs-manager": "Working"}


@app.post("/logs")
def get_logs(logs: LogSystem):
    logs_file = 'logs/logs.json'

    if os.path.exists(logs_file):
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        existing_logs = []

    new_log = logs.dict()
    existing_logs.append(new_log)

    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}


@app.get("/clear")
def clear_logs():
    logs_file = 'logs/logs.json'

    # Clear the JSON logs file
    with open(logs_file, 'w') as file:
        json.dump([], file, indent=4)

    # Remove all .png files in the logs directory
    for file_name in os.listdir('logs'):
        if file_name.endswith('.png'):
            os.remove(os.path.join('logs', file_name))

    return {"cleared": True}



@app.get("/graph")
def create_graphs():
    logs_file = 'logs/logs.json'

    if not os.path.exists(logs_file):
        return {"error": "No logs found"}

    with open(logs_file, 'r') as file:
        logs = json.load(file)

    # Plot RAM usage over time
    timestamps = [log["timestamp"] for log in logs]
    total_ram = [log["total_ram"] for log in logs]
    used_ram = [log["used_ram"] for log in logs]
    free_ram = [log["free_ram"] for log in logs]

    plt.figure(figsize=(10, 6))
    plt.plot(timestamps, total_ram, label="Total RAM")
    plt.plot(timestamps, used_ram, label="Used RAM")
    plt.plot(timestamps, free_ram, label="Free RAM")
    plt.xlabel("Timestamp")
    plt.ylabel("RAM (bytes)")
    plt.title("RAM Usage Over Time")
    plt.legend()
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig('logs/ram_usage.png')
    plt.close()

    # Plot CPU and memory usage for each process
    for log in logs:
        pids = [process["pid"] for process in log["processes"]]
        cpu_usage = [process["cpu_usage"] for process in log["processes"]]
        memory_usage = [process["memory_usage"]
                        for process in log["processes"]]

        plt.figure(figsize=(10, 6))
        plt.bar(pids, cpu_usage, label="CPU Usage (%)")
        plt.bar(pids, memory_usage, label="Memory Usage (%)", alpha=0.7)
        plt.xlabel("Process ID")
        plt.ylabel("Usage")
        plt.title(f"CPU and Memory Usage for Processes at {log['timestamp']}")
        plt.legend()
        plt.tight_layout()
        plt.savefig(
            f'logs/process_usage_{log["timestamp"].replace(":", "-")}.png')
        plt.close()

    return {"graphs_created": True}


# Serve the logs directory as static files so that images can be accessed directly
app.mount("/logs", StaticFiles(directory="logs"), name="logs")


@app.get("/view", response_class=HTMLResponse)
def view_graphs():
    ram_graph = 'logs/ram_usage.png'
    process_graphs = [
        f'logs/{f}' for f in os.listdir('logs') if f.startswith('process_usage_')]

    # Start building the HTML response
    html_content = "<html><head><title>Graphs</title></head><body>"
    html_content += "<h1>Generated Graphs</h1>"

    if os.path.exists(ram_graph):
        html_content += f"<h2>RAM Usage</h2><img src='/{
            ram_graph}' alt='RAM Usage Graph'>"

    for graph in process_graphs:
        html_content += f"<h2>Process Usage</h2><img src='/{
            graph}' alt='Process Usage Graph'>"

    html_content += "</body></html>"

    return HTMLResponse(content=html_content)
```

## Contenedor de administracion de logs

