FROM rust:1.70

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

CMD ["boa"]
