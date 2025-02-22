# Step 1: Use the official Rust image as a base image
FROM rust:latest as builder

# Step 2: Set the working directory
WORKDIR /app

# Step 3: Copy Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Step 4: Create a dummy src folder to allow Docker to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Step 5: Build the dependencies (this will cache the build step if dependencies don't change)
RUN cargo build --release || true

# Step 6: Copy the rest of the source code
COPY . .

# Step 7: Install Diesel CLI to run migrations
RUN cargo install diesel_cli --no-default-features --features postgres

# Step 10: Build the application in release mode
RUN cargo build --release

# Step 11: Set the entrypoint to your compiled binary
CMD diesel migration run && ./target/release/my_app