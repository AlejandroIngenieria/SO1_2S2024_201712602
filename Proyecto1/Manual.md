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
```bash
crontab -e
```
#### Añadimos la configuracion para 30 segundos con la **(ruta donde este el archivo que crea los contenedores)**
```bash                                                                       
* * * * * /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/2-createContainers.sh
* * * * * sleep 30; /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/scripts/2-createContainers.sh
```
#### Verificando la creacion de los contenedores
![alt text](<imgs/4. containers.png>)

# 2. Módulos de Kernel
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
