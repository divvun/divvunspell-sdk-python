FROM quay.io/pypa/manylinux2010_x86_64

# Install Rust
ADD https://sh.rustup.rs rustup-init
RUN sh rustup-init -y
ENV PATH=/root/.cargo/bin:$PATH

# Sets everything up for running libexec/build-wheels.sh:
ENV MANYLINUX=2010
ENV PLAT=manylinux${MANYLINUX}_x86_64
WORKDIR /io
