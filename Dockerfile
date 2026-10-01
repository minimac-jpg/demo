FROM debian:bookworm-slim

WORKDIR /app

# The pipeline builds the release binary in the workspace first; package it.
COPY target/release/demo /usr/local/bin/demo

CMD ["/usr/local/bin/demo"]