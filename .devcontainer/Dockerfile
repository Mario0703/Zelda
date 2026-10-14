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
    && rm -rf /var/lib/apt/lists/*

# 2. Setup Rust
# Note: Using -y to automate installation
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create workspace
RUN mkdir -p /workspace
WORKDIR /workspace

# Verify tools
RUN clang --version && llvm-config --version && rustc --version

# Ensure bash is the entrypoint and it stays open
CMD ["/bin/bash"]
