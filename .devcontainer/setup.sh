apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  helix \
  build-essential \
  openssl

curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit
