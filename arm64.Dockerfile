########### Build ###########
FROM arm64v8/rust:slim-buster AS builder

# Needed C dependencies
RUN apt update && \
    apt install -y pkg-config build-essential libssl-dev libsasl2-dev

# Run as unprivileged user
RUN useradd -ms /bin/bash producer
USER producer
WORKDIR /home/producer

COPY --chown=producer:producer src src/
COPY --chown=producer:producer Cargo.toml Cargo.toml
COPY --chown=producer:producer config.yml config.yml

RUN cargo check --manifest-path Cargo.toml
RUN cargo build --release --manifest-path Cargo.toml

########### Final image ###########
FROM arm64v8/debian:buster-slim

RUN apt update && \
    apt install -y libssl-dev libsasl2-dev ca-certificates

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /home/producer
COPY --from=builder --chown=producer:producer /home/producer/target/release/event-producer ./event-producer

ENV PRODUCER_CONFIG_FILE=config.yml
ENV RUST_LOG=NONE

EXPOSE 8081

USER producer
CMD "./event-producer"