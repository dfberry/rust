# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# docker build --no-cache -t helloworld:latest .
# docker run --rm --name helloworld1 helloworld

FROM rust as build

# create a new empty shell project
RUN USER=root cargo new --bin helloworld
WORKDIR /helloworld

# copy over your manifests
COPY ./Cargo* .

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/helloworld*
RUN cargo build --release

#--------------------------------------------------------------
# our final base
FROM rust

# copy the build artifact from the build stage
COPY --from=build /helloworld/target/release/helloworld .

# set the startup command to run your binary
CMD ["./helloworld"]