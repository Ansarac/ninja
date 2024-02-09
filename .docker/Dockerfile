FROM rust:latest

# RUN apt-get update && apt-get install -y libclang-dev

WORKDIR /workspace

# COPY Cargo.toml ./
# COPY examples ./
# RUN cargo build --release

COPY . .

RUN cargo build --release

CMD ["./target/release/ninja"]