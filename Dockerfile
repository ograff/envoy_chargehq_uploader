# Use the official Rust Docker image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the source code to the container
COPY Cargo.toml .
COPY src .

# Build the application
RUN cargo build --release

# Use a minimal Alpine-based image as the final base image
FROM alpine:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the built application from the builder stage
COPY --from=builder /app/target/release/chargehq_enphase_uploader .

# Copy the shell script to the container
COPY run.sh .

# Set the entry point for the container
CMD ["sh", "run.sh", "chargehq_enphase_uploader"]
