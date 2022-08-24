FROM rust:1.63

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["starter-snake-rust"]
