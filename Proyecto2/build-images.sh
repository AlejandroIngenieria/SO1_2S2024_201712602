#!/bin/bash

# Remove all Docker images
docker rmi -f $(docker images -a -q)

# Variables for the Docker images
GO_CLIENT_IMAGE="go-client"
RUST_CLIENT_IMAGE="rust-client"



GO_SERVER_WINNERS_IMAGE="consumer-winners"
GO_SERVER_LOSERS_IMAGE="consumer-losers"
GO_SERVER_ATLETISMO_IMAGE="go-server-atletismo"
GO_SERVER_BOXEO_IMAGE="go-server-boxeo"
GO_SERVER_NATACION_IMAGE="go-server-natacion"

DOCKERHUB_USERNAME="alejandroingenieria"
TAG="0.3"

# # Build the Docker image for the Go client
docker build -t $GO_CLIENT_IMAGE ./gRPC/go-client
# Build the Docker image for the Rust client
docker build -t $RUST_CLIENT_IMAGE ./gRPC/grpc-client

# Build the Docker image for the Go server Natacion
docker build -t $GO_SERVER_NATACION_IMAGE ./gRPC/go-server-natacion
docker build -t $GO_SERVER_WINNERS_IMAGE ./gRPC/consumer-winners
docker build -t $GO_SERVER_LOSERS_IMAGE ./gRPC/consumer-losers
# Build the Docker image for the Go server Atletismo
docker build -t $GO_SERVER_ATLETISMO_IMAGE ./gRPC/go-server-atletismo
# Build the Docker image for the Go server Boxeo
docker build -t $GO_SERVER_BOXEO_IMAGE ./gRPC/go-server-boxeo
# Build the Docker image for the Go server Natacion
docker build -t $GO_SERVER_NATACION_IMAGE ./gRPC/go-server-natacion




# Tag the Docker image
docker tag $GO_CLIENT_IMAGE "$DOCKERHUB_USERNAME/$GO_CLIENT_IMAGE:$TAG"
docker tag $RUST_CLIENT_IMAGE "$DOCKERHUB_USERNAME/$RUST_CLIENT_IMAGE:$TAG"

docker tag $GO_SERVER_WINNERS_IMAGE "$DOCKERHUB_USERNAME/$GO_SERVER_WINNERS_IMAGE:$TAG"
docker tag $GO_SERVER_LOSERS_IMAGE "$DOCKERHUB_USERNAME/$GO_SERVER_LOSERS_IMAGE:$TAG"
docker tag $GO_SERVER_ATLETISMO_IMAGE "$DOCKERHUB_USERNAME/$GO_SERVER_ATLETISMO_IMAGE:$TAG"
docker tag $GO_SERVER_BOXEO_IMAGE "$DOCKERHUB_USERNAME/$GO_SERVER_BOXEO_IMAGE:$TAG"
docker tag $GO_SERVER_NATACION_IMAGE "$DOCKERHUB_USERNAME/$GO_SERVER_NATACION_IMAGE:$TAG"

# # Push the Docker image to DockerHub
docker push "$DOCKERHUB_USERNAME/$GO_CLIENT_IMAGE:$TAG"
docker push "$DOCKERHUB_USERNAME/$RUST_CLIENT_IMAGE:$TAG"

docker push "$DOCKERHUB_USERNAME/$GO_SERVER_WINNERS_IMAGE:$TAG"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_LOSERS_IMAGE:$TAG"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_ATLETISMO_IMAGE:$TAG"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_BOXEO_IMAGE:$TAG"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_NATACION_IMAGE:$TAG"

echo "Docker images pushed successfully."