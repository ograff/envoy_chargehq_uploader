# Use the official Rust Docker image as the base image
FROM --platform=$BUILDPLATFORM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container
COPY Cargo.toml .
COPY src ./src

# Build the application
FROM --platform=$BUILDPLATFORM builder AS build-amd64
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM --platform=$BUILDPLATFORM builder AS build-arm64
RUN cargo install --target aarch64-unknown-linux-musl --path .

# Set the working directory inside the container
FROM build-$BUILDARCH AS build_out
# Copy the built application from the builder stage
# Use a minimal Alpine-based image as the final base image
FROM --platform=$TARGETPLATFORM alpine:latest
WORKDIR /app

COPY --from=build_out /app/target/release/chargehq_enphase_uploader .

# Copy the shell script to the container
COPY run.sh .

# Set the entry point for the container
CMD ["sh", "run.sh", "chargehq_enphase_uploader"]
