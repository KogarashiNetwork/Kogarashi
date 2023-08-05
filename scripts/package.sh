#!/bin/sh

docker build docker -f docker/LIGHT -t ghcr.io/kogarashinetwork/kogarashi/light
docker push ghcr.io/kogarashinetwork/kogarashi/light
