.PHONY: build
build:
	cargo build --release
	# docker build -t zeyanlin/demo_rs:v0.1.0 .

.PHONY: run
run:
	./target/release/demo_rs
