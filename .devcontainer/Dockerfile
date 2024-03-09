FROM ubuntu:latest

# Update package lists and install necessary dependencies
RUN apt-get update && \
    apt-get install -y \
    python3 \
    python3-pip \
    libsdl2-dev \
    curl

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up working directory
WORKDIR /app

# Copy your code into the container
COPY . .

# Optionally, you can install Python dependencies
RUN pip3 install -r requirements.txt

# Set up entry point or command
CMD ["/bin/bash"]
