run:
    cargo run

build:
    cargo build --release

linux:
    cargo build --release --target=

windows:
    cargo build --release --target=x86_64-pc-windows-gnu

build-all:
    cargo build
    cargo build --release
    cargo build --release --target=x86_64-pc-windows-gnu