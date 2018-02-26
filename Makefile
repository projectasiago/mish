all: empty
	RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-unknown-efi

clean:
	@rm -rf build target

.PHONY:
empty: