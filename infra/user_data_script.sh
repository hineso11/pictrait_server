mkdir -p /tmp
cd tmp

# Set up Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
chmod +x rustup.sh
./rustup.sh -y
source $HOME/.cargo/env

# Install C toolchain
sudo apt-get update
sudo apt install build-essential -y

# Get the source code
git clone https://github.com/hineso11/pictrait_server.git

# Build server
cd pictrait_server
cargo build --release

cp target/release/pictrait_server /usr/local/bin/
