FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Show what we're working with for debugging
RUN pwd && \
    ls -la && \
    echo "Content of Cargo.toml:" && \
    cat Cargo.toml

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl1.1 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/userspace /usr/local/bin/

WORKDIR /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000

CMD ["userspace"]