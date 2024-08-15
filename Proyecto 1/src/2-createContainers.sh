# ---------------------------------------------------------------------------- #
#                        Script creador de contenedores                        #
# ---------------------------------------------------------------------------- #
#!/bin/bash

# Nombres de las imágenes Docker
IMAGENES=("alto_cpu" "alto_ram" "bajo_cpu" "bajo_ram")

# Número de contenedores a crear
NUM_CONTENEDORES=10

# Intervalo en segundos entre cada ejecución
INTERVALO=30

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
        docker run -d --name ${NOMBRE_CONTAIN} ${IMAGEN}
        
        if [ $? -eq 0 ]; then
            echo "Contenedor ${NOMBRE_CONTAIN} creado exitosamente."
        else
            echo "Error al crear el contenedor ${NOMBRE_CONTAIN}."
        fi
    done
}

# Bucle principal para crear contenedores cada INTERVALO segundos
while true; do
    crear_contenedores
    echo "Esperando ${INTERVALO} segundos antes de crear más contenedores..."
    sleep $INTERVALO
done