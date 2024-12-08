# docker build --no-cache -t helloworld:latest .
# docker run --rm --name helloworld1 helloworld

# 1. This tells docker to use the Rust official image
FROM rust 

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["./target/release/helloworld"]
