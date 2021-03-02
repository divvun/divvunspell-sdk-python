#!/bin/bash
set -e -u -x

cd /io

# Compile wheels
for PYBIN in /opt/python/cp3[789]*/bin; do
    "${PYBIN}/pip" install maturin
    "${PYBIN}/maturin" build -i "${PYBIN}/python" --release
done

# Bundle external shared libraries into the wheels
for whl in wheelhouse/*.whl; do
    auditwheel repair "$whl"
done
