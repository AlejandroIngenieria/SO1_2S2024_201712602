# ---------------------------------------------------------------------------- #
#                    Creacion de las 4 imaganes principales                    #
# ---------------------------------------------------------------------------- #
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
