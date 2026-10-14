# Use a stable base image
FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# 1. Setup System Dependencies
# We combine all apt-gets into one layer to keep the image smaller
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    build-essential \
    curl \
    git \
    zstd \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. Setup Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create the workspace directory (important for devcontainer.json)
RUN mkdir -p /workspace
WORKDIR /workspace

# Verify core tools are working
RUN clang --version && llvm-config --version && rustc --version

# Default command
CMD ["/bin/bash"]
