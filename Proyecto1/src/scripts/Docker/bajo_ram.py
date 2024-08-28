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
