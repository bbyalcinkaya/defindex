CHILDREN = ../vault

default: build

all: test

build: build-children

test: build
	cargo test

build-children:
	@for dir in $(CHILDREN); do \
		$(MAKE) -C $$dir build || break; \
	done

fmt:
	cargo fmt --all --check

clean:
	cargo clean
