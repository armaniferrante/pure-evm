FROM ubuntu:18.04

ARG DEBIAN_FRONTEND=noninteractive

# Install base utilities.
RUN apt-get update -qq && apt-get upgrade -qq && apt-get install -qq \
    git curl libssl-dev pkg-config cmake zlib1g-dev \
    python jq libsqlite3-dev

ENV HOME="/root"
ENV PATH="${HOME}/.cargo/bin:${PATH}"

# Install rust and wasm-pack.
RUN curl "https://sh.rustup.rs" -sfo rustup.sh && \
    sh rustup.sh -y && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rustfmt && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install node and node globals dependencies.
RUN curl -o- https://raw.githubusercontent.com/creationix/nvm/v0.33.11/install.sh | bash
ENV NVM_DIR="${HOME}/.nvm"
RUN . $NVM_DIR/nvm.sh && \
    nvm install 10.15.3 && \
    nvm alias default node && \
    npm config set user 0 && \
    npm config set unsafe-perm true && \
    npm install -g yarn
