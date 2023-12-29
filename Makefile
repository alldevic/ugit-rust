.DEFAULT_GOAL := test
.PHONY: init clean test

init:
	cargo run init

clean:
	rm -rf .ugit

test: clean init
	cargo run hash-object test