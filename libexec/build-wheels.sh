#!/bin/bash
set -e -u -x

# Compile wheels
for PYBIN in /opt/python/cp3[789]*/bin; do
    "${PYBIN}/pip" install maturin
    "${PYBIN}/maturin" build \
        -i "${PYBIN}/python" \
        --release \
        --strip \
        --manylinux "${MANYLINUX}" `#should have been set by Dockerfile`
done
