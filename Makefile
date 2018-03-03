.PHONY:
all: build-release

.PHONY:
build:
	@cargo build

.PHONY:
build-release:
	@cargo build --release

.PHONY:
test:
	@cargo test

.PHONY:
clean:
	@rm -rf build target