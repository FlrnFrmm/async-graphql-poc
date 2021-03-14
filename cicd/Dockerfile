###################
### Build Image ### 
###################

FROM rust:latest as builder

RUN apt-get update

RUN apt-get install musl-tools -y

WORKDIR /usr/src/myapp

COPY ./Cargo.toml ./Cargo.lock ./

COPY ./src ./src

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install --target x86_64-unknown-linux-musl --path .

####################
# Deployment Image # 
####################

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend

CMD ["backend"]
