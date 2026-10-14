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

# 2. Create a non-root user 'zelda'
ARG USERNAME=zelda
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# 3. Setup Rust for the 'zelda' user
USER $USERNAME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/zelda/.cargo/bin:${PATH}"

# Create the workspace directory and set permissions
USER root
RUN mkdir -p /workspace && chown -R zelda:zelda /workspace
USER $USERNAME

WORKDIR /workspace

# Verify tools
RUN clang --version && llvm-config --version && rustc --version

CMD ["/bin/bash"]
