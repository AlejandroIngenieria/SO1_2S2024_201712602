from fastapi.responses import HTMLResponse      # type:ignore
from fastapi.staticfiles import StaticFiles     # type:ignore
import matplotlib.pyplot as plt                 # type:ignore
from fastapi import FastAPI, Request            # type:ignore
from fastapi.templating import Jinja2Templates  # type:ignore
from fastapi.responses import HTMLResponse      # type:ignore
from models.models import LogSystem
from typing import List
import json
import os

app = FastAPI()

templates = Jinja2Templates(directory="templates")

@app.get("/")
def read_root():
    return {"Logs-manager": "Working"}


@app.post("/logs")
def get_logs(logs: LogSystem):
    logs_file = 'logs/logs.json'

    if os.path.exists(logs_file):
        with open(logs_file, 'r') as file:
            existing_logs = json.load(file)
    else:
        existing_logs = []

    new_log = logs.dict()
    existing_logs.append(new_log)

    with open(logs_file, 'w') as file:
        json.dump(existing_logs, file, indent=4)

    return {"received": True}


@app.get("/clear")
def clear_logs():
    logs_file = 'logs/logs.json'

    # Clear the JSON logs file
    with open(logs_file, 'w') as file:
        json.dump([], file, indent=4)

    # Remove all .png files in the logs directory
    for file_name in os.listdir('logs'):
        if file_name.endswith('.png'):
            os.remove(os.path.join('logs', file_name))

    return {"cleared": True}


@app.get("/graph")
def create_graphs():
    logs_file = 'logs/logs.json'

    if not os.path.exists(logs_file):
        return {"error": "No logs found"}

    with open(logs_file, 'r') as file:
        logs = json.load(file)

    # Gráfico del uso de RAM a lo largo del tiempo
    timestamps = [log["timestamp"] for log in logs]
    total_ram = [log["total_ram"] for log in logs]
    used_ram = [log["used_ram"] for log in logs]
    free_ram = [log["free_ram"] for log in logs]

    plt.figure(figsize=(12, 6))
    plt.plot(timestamps, total_ram, label="Total RAM",
             color="blue", linewidth=2)
    plt.plot(timestamps, used_ram, label="Used RAM",
             color="red", linestyle="--", linewidth=2)
    plt.plot(timestamps, free_ram, label="Free RAM",
             color="green", linestyle=":", linewidth=2)
    plt.xlabel("Timestamp", fontsize=12)
    plt.ylabel("RAM (bytes)", fontsize=12)
    plt.title("RAM Usage Over Time", fontsize=14)
    plt.legend()
    plt.xticks(rotation=45)
    plt.grid(True)
    plt.tight_layout()
    plt.savefig('logs/ram_usage.png')
    plt.close()

    # Gráficos para los procesos individuales
    for log in logs:
        pids = [process["pid"] for process in log["processes"]]
        names = [process["name"] for process in log["processes"]]
        vsz = [process["vsz"]
               for process in log["processes"]]  # Tamaño de memoria virtual
        rss = [process["rss"]
               for process in log["processes"]]  # Tamaño de memoria residente
        cpu_usage = [process["cpu_usage"] for process in log["processes"]]
        memory_usage = [process["memory_usage"]
                        for process in log["processes"]]
        cmdline = [process["cmdline"] for process in log["processes"]]

        # Configuración del gráfico
        fig, ax1 = plt.subplots(figsize=(12, 8))

        # Gráfico de uso de CPU y memoria en porcentaje
        bar_width = 0.35
        index = range(len(pids))

        ax1.bar(index, cpu_usage, bar_width,
                label="CPU Usage (%)", color='blue', alpha=0.7)
        ax1.bar([i + bar_width for i in index], memory_usage, bar_width,
                label="Memory Usage (%)", color='orange', alpha=0.7)

        ax1.set_xlabel('Process ID', fontsize=12)
        ax1.set_ylabel('Usage (%)', fontsize=12)
        ax1.set_title(f"Resource Usage for Processes at {log['timestamp']}", fontsize=14)
        ax1.set_xticks([i + bar_width / 2 for i in index])
        ax1.set_xticklabels([f"{name} (PID: {pid})" for name, pid in zip(
            names, pids)], rotation=45, ha="right", fontsize=10)
        ax1.legend()

        # Anotaciones de detalles adicionales
        for i in index:
            ax1.text(
                i, cpu_usage[i] + 0.5, f"VSZ: {vsz[i]} KB", ha="center", va="bottom", fontsize=9)
            ax1.text(i + bar_width, memory_usage[i] + 0.5, f"RSS: {rss[i]} KB", ha="center", va="bottom", fontsize=9)

        plt.tight_layout()
        plt.savefig(
            f'logs/process_usage_{log["timestamp"].replace(":", "-")}.png')
        plt.close()

    return {"graphs_created": True}



# Serve the logs directory as static files so that images can be accessed directly
app.mount("/logs", StaticFiles(directory="logs"), name="logs")


@app.get("/view", response_class=HTMLResponse)
def view_graphs(request: Request):
    # Asume que 'logs' está en la raíz del directorio de trabajo del contenedor
    ram_graph = '/logs/ram_usage.png' if os.path.exists(
        '/code/logs/ram_usage.png') else None
    process_graphs = [
        f'/logs/{f}' for f in os.listdir('/code/logs') if f.startswith('process_usage_')
    ]

    return templates.TemplateResponse("view_graphs.html", {"request": request, "ram_graph": ram_graph, "process_graphs": process_graphs})
