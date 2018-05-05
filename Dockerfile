FROM rust:latest

WORKDIR /usr/src/paste-acm
COPY . .

RUN cargo install
CMD ["paste-acm"]
