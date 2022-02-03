FROM rust:1.57 as build

# create a new empty shell project
RUN USER=root cargo new --bin rocket_server 
WORKDIR /rocket_server 

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/rocket_server*
RUN cargo build --release

# our final base
FROM rust:1.57-slim-buster
RUN mkdir /app
WORKDIR /app
COPY Rocket.toml .

# copy the build artifact from the build stage
COPY --from=build /rocket_server/target/release/rocket_server .

# set the startup command to run your binary
CMD ["./rocket_server"]