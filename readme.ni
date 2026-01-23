cargo install espup --locked
cargo install cargo-espflash --locked
cargo install cargo-espmonitor
espup install
echo 'source $HOME/export-esp.sh' >> ~/.bashrc
source ~/.bashrc
cargo build --features clipboard,wifi
