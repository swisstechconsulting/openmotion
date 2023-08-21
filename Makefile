CHIP_NAME?=STM32F407VGTx
TARGET?=thumbv7em-none-eabihf
TOOLS_BIN=tools/cargo/bin
BINARY=openmotion

default: lint test build

.PHONY: lint
lint:
	cargo fmt
	cargo clippy

.PHONY: test
test:
	cargo test --target $(TARGET)

.PHONY: build
build:
	cargo build --release --target $(TARGET)
	cargo build --release

.PHONY: run
run: $(TOOLS_BIN)/probe-rs
	cargo readobj --bin $(BINARY) -- --file-headers
	cargo size --bin $(BINARY) --release -- -A
	@echo "Flashing into RAM (debug build)"
	cargo flash --chip "${CHIP_NAME}"

.PHOZNy: debug
debug:
	# https://docs.rust-embedded.org/book/intro/install/macos.html
	# openocd -f interface/stlink.cfg -f target/stm32f4x.cfg
	# brew install openocd
	# brew install qemu
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg

.PHONY: flash
flash: probe-target/STM32F4_Series.yaml $(TOOLS_BIN)/probe-rs
	@echo "Flashing into Flash (release build)"
	cargo flash --release --chip "${CHIP_NAME}"

.PHONY: tools
tools: $(TOOLS_BIN)/probe-rs
	rustup update
	rustup component add rustfmt
	rustup component add clippy
	
tools/cargo/%:
	cargo install --root tools/cargo "$(@F)"

$(TOOLS_BIN)/probe-rs: $(TOOLS_BIN)/cargo-binutils $(TOOLS_BIN)/cargo-expand
	rustup component add llvm-tools-preview
	rustup target add $(TARGET)
	# cargo install --root tools/cargo cargo-binutils cargo-embed cargo-flash cargo-expand
	cargo install --root tools/cargo "$(@F)" --features cli

probe-target/%_Series.yaml: $(TOOLS_BIN)/target-gen
	@mkdir -p $(@D)
	$(TOOLS_BIN)/target-gen arm -f "$*" "$(@D)"

.PHONY: clean
clean:
	cargo clean
	rm -rf $(TOOLS_BIN) probe-target/*.yaml

