from fastapi import FastAPI  # type: ignore
import os
import json
from typing import List
from models.models import LogSystem

app = FastAPI()


@app.get("/")
def read_root():
    return {"Logs-manager" : "Working"}


@app.post("/logs")
def get_logs(logs: LogSystem):
    logs_file = 'logs/logs.json'

    # Checamos si existe el archivo logs.json
    if os.path.exists(logs_file):
        # Leemos el archivo logs.json
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        # Sino existe, creamos una lista vacía
        existing_logs = []

    # Agregamos el nuevo log a la lista existente
    new_log = logs.dict()
    existing_logs.append(new_log)

    # Escribimos la lista de logs en el archivo logs.json
    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}

