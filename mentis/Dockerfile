# Official Rust Image
FROM rust:latest

# Set working dir in the container
WORKDIR /app

# Copy project files into the container
COPY . .

# Build the application in release config
RUN cargo build --release

# Specify command to run application
CMD ["/app/target/release/mentis"]