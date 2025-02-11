FROM rust:1.75-alpine as builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    gcc \
    make \
    git \
    perl \
    alpine-sdk

# Create a new empty shell project
WORKDIR /usr/src/app

# First, copy only Cargo.toml and Cargo.lock for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# Now copy the real source code
COPY . .

# Show what we're working with
RUN pwd && \
    ls -la && \
    echo "Content of Cargo.toml:" && \
    cat Cargo.toml

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    libgcc \
    openssl \
    postgresql-libs

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/userspace /usr/local/bin/

WORKDIR /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000

CMD ["userspace"]