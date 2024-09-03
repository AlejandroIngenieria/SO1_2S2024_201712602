from fastapi import FastAPI                     #type:ignore
from fastapi.responses import HTMLResponse      # type:ignore
from fastapi.staticfiles import StaticFiles     # type:ignore
import os
import json
from typing import List
from models.models import LogSystem
import matplotlib.pyplot as plt #type:ignore

app = FastAPI()


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

    # Plot RAM usage over time
    timestamps = [log["timestamp"] for log in logs]
    total_ram = [log["total_ram"] for log in logs]
    used_ram = [log["used_ram"] for log in logs]
    free_ram = [log["free_ram"] for log in logs]

    plt.figure(figsize=(10, 6))
    plt.plot(timestamps, total_ram, label="Total RAM")
    plt.plot(timestamps, used_ram, label="Used RAM")
    plt.plot(timestamps, free_ram, label="Free RAM")
    plt.xlabel("Timestamp")
    plt.ylabel("RAM (bytes)")
    plt.title("RAM Usage Over Time")
    plt.legend()
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.savefig('logs/ram_usage.png')
    plt.close()

    # Plot CPU and memory usage for each process
    for log in logs:
        pids = [process["pid"] for process in log["processes"]]
        cpu_usage = [process["cpu_usage"] for process in log["processes"]]
        memory_usage = [process["memory_usage"]
                        for process in log["processes"]]

        plt.figure(figsize=(10, 6))
        plt.bar(pids, cpu_usage, label="CPU Usage (%)")
        plt.bar(pids, memory_usage, label="Memory Usage (%)", alpha=0.7)
        plt.xlabel("Process ID")
        plt.ylabel("Usage")
        plt.title(f"CPU and Memory Usage for Processes at {log['timestamp']}")
        plt.legend()
        plt.tight_layout()
        plt.savefig(
            f'logs/process_usage_{log["timestamp"].replace(":", "-")}.png')
        plt.close()

    return {"graphs_created": True}


# Serve the logs directory as static files so that images can be accessed directly
app.mount("/logs", StaticFiles(directory="logs"), name="logs")


@app.get("/view", response_class=HTMLResponse)
def view_graphs():
    ram_graph = 'logs/ram_usage.png'
    process_graphs = [
        f'logs/{f}' for f in os.listdir('logs') if f.startswith('process_usage_')]

    # Start building the HTML response
    html_content = "<html><head><title>Graphs</title></head><body>"
    html_content += "<h1>Generated Graphs</h1>"

    if os.path.exists(ram_graph):
        html_content += f"<h2>RAM Usage</h2><img src='/{
            ram_graph}' alt='RAM Usage Graph'>"

    for graph in process_graphs:
        html_content += f"<h2>Process Usage</h2><img src='/{
            graph}' alt='Process Usage Graph'>"

    html_content += "</body></html>"

    return HTMLResponse(content=html_content)
