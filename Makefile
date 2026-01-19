APP_NAME=mpfree

all: run

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

windows:
	cargo build --release --target x86_64-pc-windows-gnu

clean:
	cargo clean