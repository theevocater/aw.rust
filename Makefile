all: test

test:
	rustc src/aw/lib.rs --test -o build/aw/aw-tests
	./build/aw/aw-tests

