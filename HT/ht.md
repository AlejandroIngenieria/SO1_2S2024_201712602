# Hoja de trabajo - Sistemas operativos 1
> ## Creacion del custer
En el proyecto **usac-nginx-cluster** creamos el cluster con el nombre **nginx-cluster**
![alt text](<imgs/1 - creacion del cluster.png>)

> ## Aplicacion en GO
- [main.go](go-app/main.go)
- [Dockerfile](go-app/Dockerfile)

### Construccion del contenedor
```bash
docker build -t gcr.io/usac-nginx-cluster/go-app .
```
![alt text](<imgs/2- construccion del contenedor.png>)

### Subir la imagen a GCR

```bash
docker push gcr.io/usac-nginx-cluster/go-app
```

![alt text](<imgs/4 - subir imagen a GCR.png>)

> ## Locust
- [main.py](locust/main.py)
- [data.json](locust/data.json)

### Ejecucion de locust

```bash
locust -f main.py
```

![alt text](<imgs/3 - configuracion de locust.png>)

> ## Deployment
- [deployment.yaml](k8s/deployment.yaml)

![alt text](<imgs/5 - deployment correcto.png>)

> ## Funcionamiento del proyecto

### Enviamos trafico con locust
![alt text](<imgs/6 - trafico en locust.png>)

### Logs de envios
![alt text](<imgs/7 - logs de locust.png>)

> ## Eliminacion del cluster
```bash
gcloud container clusters delete nginx-cluster --zone us-central1-a
```