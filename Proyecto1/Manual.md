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
# 3. Servicio de Rust
