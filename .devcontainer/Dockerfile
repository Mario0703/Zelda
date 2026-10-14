# Use a stable base image
FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# 1. Setup Clang and LLVM 
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    build-essential \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# 2. Setup Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# seup open code
RUN curl -fsSL https://opencode.ai/v2/install | bash

# 3. Set Environment Variables
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify installations
RUN clang --version && llvm-config --version && rustc --version

# Default command
CMD ["bash"]
