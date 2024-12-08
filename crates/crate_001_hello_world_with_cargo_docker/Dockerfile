# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# docker build --no-cache -t helloworld:latest .
# docker run --rm --name helloworld1 helloworld

# Rust as the base image
FROM rust

# 1. Create a new empty shell project
RUN USER=root cargo new --bin helloworld
WORKDIR /helloworld

# 2. Copy our manifests
COPY ./Cargo* .

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/helloworld*
RUN cargo install --path .

CMD ["helloworld"]
