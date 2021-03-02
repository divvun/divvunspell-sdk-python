#!/bin/bash

set -eux

DOCKER_IMAGE=quay.io/pypa/manylinux2010_x86_64
PLAT=manylinux2010_x86_64

docker pull "${DOCKER_IMAGE}"
docker run --rm -e "${PLAT}" -v "$(pwd):/io" "${DOCKER_IMAGE}" /io/libexec/build-wheels.sh
