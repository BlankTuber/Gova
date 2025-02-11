FROM rust:1.75-alpine as builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    gcc

# Set up the build environment
WORKDIR /usr/src/app
COPY . .

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