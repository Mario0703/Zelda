FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

# 1. Setup System Dependencies
RUN apt-get update && apt-get install -y \
    clang llvm build-essential curl git zstd ca-certificates sudo \
    && rm -rf /var/lib/apt/lists/*

# 2. Setup User and Group
RUN groupadd -g 1000 compilerGroup && \
    useradd -m -u 1000 -g compilerGroup -s /bin/bash compilerUser

# Allow sudo for the postCreateCommand in devcontainer.json
RUN echo "compilerUser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# 3. Install tools as compilerUser
USER compilerUser
WORKDIR /home/compilerUser

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install openCode
RUN curl -fsSL https://opencode.ai/v2/install | bash

# Update PATH to include Rust and potentially openCode
ENV PATH="/home/compilerUser/.cargo/bin:/home/compilerUser/.local/bin:${PATH}"

# 4. Final Setup
USER root
RUN mkdir -p /workspace && chown compilerUser:compilerGroup /workspace
USER compilerUser
WORKDIR /workspace

CMD ["/bin/bash"]
