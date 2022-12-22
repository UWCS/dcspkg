FROM rust:1 as builder

# create empty projects
RUN USER=root cargo new --bin dcspkg-server
WORKDIR /dcspkg-server

# copy package manifest in 
COPY ./dcspkg-server/Cargo.toml ./Cargo.toml

# path the package manifest to fetch dcspkg lib from github
RUN sed -i 's@{ path = "../dcspkg" }@{ git = "https://github.com/UWCS/dcspkg" }@g' Cargo.toml

# build only dependancies to cache them
RUN cargo build --release
RUN rm src/*.rs

# copy source code in
COPY ./dcspkg-server/src ./src

# build our code
RUN rm ./target/release/deps/dcspkg*
RUN cargo build --release


# new base, slimmer, no toolchains
FROM debian:bullseye-slim
COPY --from=builder /dcspkg-server/target/release/dcspkg_server .

CMD [ "./dcspkg_server" ]
