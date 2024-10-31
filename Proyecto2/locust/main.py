import json
from random import randrange
from locust import HttpUser, between, task

debug = False
host = "http://34.56.128.79.nip.io"  # Dirección completa de tu Ingress

def printDebug(msg):
    if debug:
        print(msg)

class Reader():
    def __init__(self):
        self.array = []

    def pickRandom(self):
        length = len(self.array)

        if length > 0:
            random_index = randrange(0, length - 1) if length > 1 else 0
            return self.array.pop(random_index)
        else:
            print(">> Reader: No encontramos ningún valor o registro en el archivo.")
            return None

    def load(self, filename):
        print(f">> Reader: Iniciando la lectura del archivo {filename}.")
        try:
            with open(filename, 'r') as data_file:
                self.array = json.loads(data_file.read())
        except Exception as error:
            print(f'>> Reader: No se cargaron los datos desde {filename}, error: {error}')

class MessageTraffic(HttpUser):
    host = "http://34.56.128.79.nip.io"  # Dirección completa de tu Ingress
    wait_time = between(0.1, 0.9)
    
    # Instancias de Reader para cada archivo
    agronomia_reader = Reader()
    ingenieria_reader = Reader()
    
    # Cargar datos de cada archivo
    agronomia_reader.load("agronomia.json")
    ingenieria_reader.load("ingenieria.json")

    def on_start(self):
        print(">> MessageTraffic: Iniciamos el envío de tráfico")

    @task
    def post_student(self):
        # Elegir aleatoriamente entre los datos de agronomía e ingeniería
        if randrange(2) == 0:  # 0 para Agronomía, 1 para Ingeniería
            random_data = self.agronomia_reader.pickRandom()
            endpoint = "/Agronomia"
        else:
            random_data = self.ingenieria_reader.pickRandom()
            endpoint = "/Ingenieria"
        
        # Enviar la petición si hay datos
        if random_data is not None:
            data_to_send = json.dumps(random_data)
            printDebug(data_to_send)
            self.client.post(endpoint, json=random_data)
        else:
            print(f">> MessageTraffic: No hay más registros para enviar a {endpoint}.")
