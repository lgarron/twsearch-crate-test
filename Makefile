.PHONY: dev
dev:
	cargo run --release --

.PHONY: clean
clean:
	rm -rf ./target

.PHONY: reset
reset: clean
