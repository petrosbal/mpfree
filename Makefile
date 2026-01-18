APP_NAME=mpfree

all: run

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean