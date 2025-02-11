FROM rust:1.75-alpine as builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    gcc \
    make \
    git

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Show directory contents for debugging
RUN ls -la

# Show Cargo.toml contents
RUN cat Cargo.toml

# Build with verbose output
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --verbose

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