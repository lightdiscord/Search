#!/usr/bin/env bash

set -e

cweb_version=0.6.16
cweb=https://github.com/koute/cargo-web/releases/download/$cweb_version/cargo-web-x86_64-unknown-linux-gnu.gz
curl -Lo cargo-web.gz $cweb
gunzip cargo-web.gz
chmod u+x cargo-web

curl https://sh.rustup.rs -sSf | sh -s - --default-toolchain nightly -y
source ~/.cargo/env
rustup target add wasm32-unknown-unknown

./cargo-web deploy --target=wasm32-unknown-unknown
