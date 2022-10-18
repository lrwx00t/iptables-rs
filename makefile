BINARY_NAME=iptables-rs

build:
	cargo build -p iptables-rs

run:
	sudo ./target/debug/${BINARY_NAME}

build_and_run: build run

clean:
	cargo clean
	rm -f ${BINARY_NAME}

test:
	cargo test
