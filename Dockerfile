FROM debian:bookworm-slim

WORKDIR /app

# The pipeline builds the release binary in the workspace first; package it.
# chmod restores the exec bit (img's COPY doesn't always preserve it).
COPY target/release/demo /usr/local/bin/demo
RUN chmod +x /usr/local/bin/demo

CMD ["/usr/local/bin/demo"]