all: empty
	RUST_TARGET_PATH=$(shell pwd) xargo build --target x86_64-unknown-efi
	
test: empty
	RUST_TARGET_PATH=$(shell pwd) xargo test --target x86_64-unknown-efi

clean:
	@rm -rf build target

.PHONY:
empty: