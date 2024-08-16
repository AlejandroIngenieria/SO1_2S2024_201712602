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
### Contenedores de alto consumo
#### Alto consumo de RAM
```Dockerfile
FROM ubuntu:latest
RUN apt-get update && apt-get install -y stress
CMD ["stress", "--cpu", "4"]
```
* **Base:** ubuntu:latest
* **Paquete instalado:** stress
* **Comando de Ejecución:**: stress con opción para usar 4 núcleos de CPU.
#### Alto consumo de CPU
```Dockerfile
FROM ubuntu:latest
RUN apt-get update && apt-get install -y stress
CMD ["stress", "--vm", "2", "--vm-bytes", "512M"]
```
* **Base:** ubuntu:latest
* **Paquete instalado:** stress
* **Comando de Ejecución:**: stress con opción para usar 2 procesos de memoria con 512MB cada uno.
### Contenedores de bajo consumo
#### Bajo consumo de CPU
```Dockerfile
FROM ubuntu:latest
CMD ["sleep", "infinity"]
```
* **Base:** ubuntu:latest
* **Paquete instalado:** Ninguno adicional.
* **Comando de Ejecución:**: sleep con tiempo infinito, lo que mantiene el contenedor en ejecución sin hacer nada.
#### Bajo consumo de RAM
```Dockerfile
FROM ubuntu:latest
CMD ["sh", "-c", "while :; do echo 'bajo consumo'; sleep 60; done"]
```
* **Base:** ubuntu:latest
* **Paquete instalado:** Ninguno adicional.
* **Comando de Ejecución:**: Un bucle que imprime "bajo consumo" cada 60 segundos, simulando baja actividad.

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
        echo "Creando contenedor ${NOMBRE_CONTAIN} usando la imagen ${IMAGEN}..."
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
### Verificando la creacion de los contenedores
![alt text](<imgs/4. containers.png>)

## Comando para detener contenedores
```bash
docker stop $(docker ps --filter "ancestor=bajo_ram" -q) $(docker ps --filter "ancestor=alto_ram" -q) $(docker ps --filter "ancestor=alto_cpu" -q) $(docker ps --filter "ancestor=bajo_cpu" -q)
```
## Comando para eliminar contenedores
```bash
docker rm $(docker ps -a --filter "ancestor=bajo_ram" -q) $(docker ps -a --filter "ancestor=alto_ram" -q) $(docker ps -a --filter "ancestor=alto_cpu" -q) $(docker ps -a --filter "ancestor=bajo_cpu" -q)
```

## Configurar Cron Job
```bash
crontab -e
```
#### Añadimos la configuracion para 30 segundos con la **(ruta donde este el archivo que crea los contenedores)**
```bash                                                                       
* * * * * /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/2-createContainers.sh
* * * * * sleep 30; /home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/src/2-createContainers.sh
```


# 2. Módulos de Kernel
# 3. Servicio de Rust
