#!/bin/bash

set -eux

DOCKER_IMAGE=build-rusty-wheels

docker build . --tag $DOCKER_IMAGE
docker run --rm -v "$(pwd):/io" $DOCKER_IMAGE /io/libexec/build-wheels.sh
