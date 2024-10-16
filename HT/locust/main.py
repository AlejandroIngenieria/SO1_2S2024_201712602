import json
from locust import HttpUser, task

class MyUser(HttpUser):
    # Cargamos los datos del archivo al inicializar la clase
    def on_start(self):
        self.load_data()

    def load_data(self):
        try:
            with open("data.json", 'r') as data_file:
                self.payload = json.load(data_file)  # Cargar el contenido del archivo en un atributo
                print("Datos cargados: ", self.payload)  # Verificar que se hayan cargado los datos
        except Exception as e:
            print(f"Error al cargar los datos: {e}")
            self.payload = []  # Si hay un error, inicializa la lista como vacía

    @task
    def send_data(self):
        if self.payload:
            # Enviar todo el array de estudiantes como un único payload
            with self.client.post("/", json=self.payload, catch_response=True) as response:
                if response.status_code == 200:
                    print("Response: ", response.text)  # Imprime la respuesta en la consola
                else:
                    response.failure("Request failed with status code: {}".format(response.status_code))
        else:
            print("No hay datos disponibles para enviar.")
