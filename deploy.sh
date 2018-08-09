#!/bin/bash
VERSION=1.0.0
SERVER_PORT=8080
OUT_PORT=8080
CONTAINER_NAME=kontakt-server
MEMORY_LIMIT=300m # im mb

echo 'Starting'
echo 'Building Docker Image, wait ..'
echo 'Removing old Container if any ! ..'
sudo docker build -t shekohex/$CONTAINER_NAME:$VERSION .
sudo docker stop $CONTAINER_NAME
sudo docker rm $CONTAINER_NAME
sudo docker run -d \
    --restart=unless-stopped \
    -p $OUT_PORT:$SERVER_PORT \
    --memory=$MEMORY_LIMIT \
    --memory-swap=$MEMORY_LIMIT \
    --name $CONTAINER_NAME \
    shekohex/$CONTAINER_NAME:$VERSION

sudo docker logs $CONTAINER_NAME
echo 'Container Name:' $CONTAINER_NAME
echo 'Run `$ docker logs' $CONTAINER_NAME '-f` to see logs'
echo 'Done!'
