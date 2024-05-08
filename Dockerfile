FROM rust:1.48

RUN USER=root cargo new --bin streamline_workspace
WORKDIR /streamline_workspace

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/streamline_workspace*
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/streamline_workspace"]