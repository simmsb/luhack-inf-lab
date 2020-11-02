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
ENV FLAG_0=$FLAG_0
ENV FLAG_1=$FLAG_1
ENV ROOT_PW=ROOT_PW

RUN apk add util-linux python3 openssh-keygen

COPY --from=build-rust /root/.cargo/bin/server /usr/local/bin/server
COPY startup.sh /tmp/startup.sh

CMD /tmp/startup.sh
