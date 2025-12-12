# .devcontainer/Dockerfile
FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

RUN apt-get update && \
    apt-get install -y \
        python3 \
        python3-pip \
        git \
        curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
