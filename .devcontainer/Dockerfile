FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# 1. Setup System Dependencies
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    build-essential \
    curl \
    git \
    zstd \
    ca-certificates \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# 2. Setup non-root user 'compilerUser'
# We create the group and user.
# Note: VS Code Dev Containers automatically tries to match the UID/GID of your host user.
RUN groupadd -g 1000 compilerGroup && \
    useradd -m -u 1000 -g compilerGroup -s /bin/bash compilerUser

# Give compilerUser sudo access without password
RUN echo "compilerUser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# 3. Setup Rust for compilerUser
USER compilerUser
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/compilerUser/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /workspace

# Verify tools
RUN clang --version && llvm-config --version && rustc --version

CMD ["/bin/bash"]
