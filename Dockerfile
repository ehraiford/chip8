FROM python:3.12

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update && apt-get install -y \
    libsdl2-dev \
    && rm -rf /var/lib/apt/lists/*

# Install VS Code server
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    gnupg \
    && curl -fsSL https://code-server.dev/install.sh | sh

WORKDIR /app


EXPOSE 8080
ENTRYPOINT ["code-server", "--host", "0.0.0.0", "--port", "8080"]
