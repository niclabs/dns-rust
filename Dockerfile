# Rust official image
FROM rust

# Set working directory
WORKDIR /home/dns-rust

# Copy the project
COPY . .

# Resolver port
EXPOSE 58396

RUN cargo install --path .

