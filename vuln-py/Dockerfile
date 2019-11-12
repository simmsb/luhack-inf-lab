# python exec vuln and stuff

FROM clux/muslrust:latest AS build-rust

WORKDIR /opt/server/
COPY server/Cargo.toml server/Cargo.lock ./

COPY server/src ./src

RUN cargo build --release
RUN cargo install --path .

FROM alpine:latest

ARG flag_0
ARG flag_1
ARG root_pw

RUN apk add util-linux python3 openssh-keygen

COPY --from=build-rust /root/.cargo/bin/server /usr/local/bin/server

RUN adduser -g "" smooth -D -s /bin/sh
RUN passwd root -d $root_pw && ssh-keygen -f /home/smooth/root -N $root_pw

RUN su - smooth -c "echo $flag_0 > /home/smooth/flag.txt"
RUN echo $flag_1 > /root/flag.txt

CMD su - smooth -c "server"
