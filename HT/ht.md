# Hoja de trabajo - Sistemas operativos 1
> ## Creacion del custer
```bash
gcloud container clusters create ht-cluster \
    --zone us-central1-a \
    --num-nodes=1 \
    --machine-type=e2-small \
    --disk-size=20GB
```

En el proyecto **olimpiadas-usac** creamos el cluster con el nombre **nginx-cluster**
![alt text](<imgs/1 - creacion del cluster.png>)

> ## Aplicacion en GO
- [main.go](go-app/main.go)
- [Dockerfile](go-app/Dockerfile)

### Construccion del contenedor
```bash
docker build -t gcr.io/olimpiadas-usac/go-app .
```
![alt text](<imgs/2- construccion del contenedor.png>)

### Subir la imagen a GCR

```bash
docker push gcr.io/olimpiadas-usac/go-app
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
```bash
kubectl apply -f [archivo.yaml]
```
- [deployment.yaml](k8s/deployment.yaml)
- [service.yaml](k8s/service.yaml)
- [ingress.yaml](k8s/ingress.yaml)

![alt text](<imgs/5 - subiendo YAML files.png>)

![alt text](<imgs/6 - uploads.png>)

> ## Funcionamiento del proyecto

### Enviamos trafico con locust
![alt text](<imgs/7 - trafico en locust.png>)

### Logs de envios
![alt text](<imgs/8 - logs de locust.png>)

> ## Eliminacion del cluster
```bash
gcloud container clusters delete nginx-cluster --zone us-central1-a
```