# python exec vuln and stuff

FROM clux/muslrust:latest AS build-rust

WORKDIR /opt/server/

COPY server/Cargo.toml .
COPY server/Cargo.lock .
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY server/src src

RUN cargo build --release
RUN cargo install --path . --verbose

FROM alpine:latest

ARG FLAG_0
ARG FLAG_1
ARG ROOT_PW

RUN apk add util-linux python3 openssh-keygen

COPY --from=build-rust /root/.cargo/bin/server /usr/local/bin/server

RUN adduser -g "" smooth -D -s /bin/sh
RUN passwd root -d $ROOT_PW && ssh-keygen -f /home/smooth/root -N $ROOT_PW

RUN su - smooth -c "echo $FLAG_0 > /home/smooth/flag.txt"
RUN echo $FLAG_1 > /root/flag.txt

CMD ["su", "-", "smooth", "-c", "server"]
