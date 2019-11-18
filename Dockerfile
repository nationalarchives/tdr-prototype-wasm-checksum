FROM rust:1.38
WORKDIR /usr/src/checksumcalc
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# Install the latest version of Rust to get support for async-await:
# https://blog.rust-lang.org/2019/09/30/Async-await-hits-beta.html
RUN rustup toolchain install nightly
RUN rustup default nightly
