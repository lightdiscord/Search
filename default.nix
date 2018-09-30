with import <nixpkgs> {
  overlays = map (uri: import (fetchTarball uri)) [
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  ];
};

let
    date = "2018-10-03";
    channel = "nightly";
in stdenv.mkDerivation {
  name = "rust-wasm";
  buildInputs = [
    cargo-web
    ((rustChannelOf { inherit date channel; }).rust.override {
      targets = ["wasm32-unknown-unknown"];
    })
  ];
}
