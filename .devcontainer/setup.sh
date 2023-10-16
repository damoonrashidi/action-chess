apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  build-essential \
  openssl

sudo add-apt-repository ppa:maveonair/helix-editor
sudo apt update
sudo apt install helix

curl https://sh.rustup.rs -sSf | sh -s -- -y

cargo install cargo-expand
cargo install cargo-edit
