
.phony all


all: build

build:
	cargo build --release

run:
	cargo run

install:
	sudo cp target/release/startw /usr/local/bin

clean:
	cargo clean
