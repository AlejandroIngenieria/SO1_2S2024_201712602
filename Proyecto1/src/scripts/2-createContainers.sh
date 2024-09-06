#!/bin/bash

# Nombres de las imágenes Docker
IMAGENES=("alto_cpu" "alto_ram" "bajo_cpu" "bajo_ram")

# Número de contenedores a crear
NUM_CONTENEDORES=10

# Ruta del archivo de log
LOG_FILE="/home/josue/Escritorio/so1_laboratorio/actividades/Proyecto1/log_ejecucion.log"

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
        
        # Crear el contenedor y capturar la salida
        OUTPUT=$( /usr/bin/docker run -d --name ${NOMBRE_CONTAIN} ${IMAGEN} 2>&1 )
        if [ $? -eq 0 ]; then
            echo "Contenedor ${NOMBRE_CONTAIN} creado exitosamente." >> $LOG_FILE
        else
            echo "Error al crear el contenedor ${NOMBRE_CONTAIN}: $OUTPUT" >> $LOG_FILE
        fi
    done
}

# Ejecutar la función para crear contenedores
crear_contenedores
