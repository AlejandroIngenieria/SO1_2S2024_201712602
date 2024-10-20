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

> ## k8s
```bash
kubectl apply -f [archivo.yaml]
```
- [deployment.yaml](k8s/deployment.yaml)
- [service.yaml](k8s/service.yaml)
- [ingress.yaml](k8s/ingress.yaml)

### Para que funcione el ingress
```bash
kubectl create ns nginx-ingress
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx 
helm repo update 
helm install nginx-ingress ingress-nginx/ingress-nginx -n nginx-ingress

kubectl get services -n nginx-ingress #Con este comando obtenemos el host para el ingress
```

### Ingreso de los archivos al cluster
```bash
kubectl apply -f nombre_del_archivo
```

![alt text](<imgs/5 - subiendo YAML files.png>)

### Corroboramos que todo este funcionando

![alt text](<imgs/6 - uploads.png>)

> ## Funcionamiento del proyecto

### Iniciamos Locust
```bash
locust -f main.py --host http://104.197.151.22.nip.io
```

### Ingresamos el puerto
![alt text](<imgs/7 - configuracion locust.png>)

### Enviamos trafico con locust
![alt text](<imgs/8 - trafico en locust.png>)

### Logs de envios
![alt text](<imgs/9 - logs de locust.png>)

### Resumen de traffico
![alt text](<imgs/10 - resultados locust.png>)

> ## Eliminacion del cluster
```bash
gcloud container clusters delete ht-cluster --zone us-central1-a
```