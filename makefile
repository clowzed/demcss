all:

build:
	cargo build --release

publish:
	surge
