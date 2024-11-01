# Para ver kafka
kubectl -n kafka run kafka-consumer -ti --image=quay.io/strimzi/kafka:0.43.0-kafka-3.8.0 --rm=true --restart=Never -- bin/kafka-console-consumer.sh --bootstrap-server my-cluster-kafka-bootstrap:9092 --topic winners --from-beginning

# Redis password
adde8k7STR
<!-- Ingresar a la consola -->
kubectl run -it --rm --namespace default redis-client --image=bitnami/redis:latest -- bash
<!-- Ingresar a la base de datos -->
redis-cli -h my-release-redis-master -p 6379 -a adde8k7STR

# Grafana
<!-- Ver el puerto de grafana-node-port-service -->
kubectl get svc
<!-- Obtener el EXTERNAL-IP correcto -->
kubectl get nodes -o wide
<!-- Credenciales -->
admin
prom-operator