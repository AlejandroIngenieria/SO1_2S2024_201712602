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
