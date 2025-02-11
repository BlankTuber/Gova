FROM rust:1.75-alpine as builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc openssl

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/userspace /usr/local/bin/

# Copy .env file
COPY .env /usr/local/bin/

WORKDIR /usr/local/bin
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 3000

CMD ["userspace"]