FROM rust:latest
WORKDIR /opt
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends