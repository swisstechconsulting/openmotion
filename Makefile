CHIP_NAME?=STM32F407VGTx
TARGET?=thumbv7em-none-eabihf
TOOLS_BIN=target/tools/cargo/bin
BINARY=openmotion

default: lint test build

.PHONY: lint
lint:
	cargo fmt
	cargo clippy

.PHONY: test
test: tools
	cargo test --target $(TARGET)

.PHONY: build
build:
	cargo build --release --target $(TARGET)

.PHONY: run
run: $(TOOLS_BIN)/probe-rs
	@echo "Flashing into RAM (debug build)"
	cargo embed --chip "${CHIP_NAME}"

.PHONY: flash
flash: $(TOOLS_BIN)/probe-rs $(TOOLS_BIN)/cargo-binutils
	cargo readobj --bin $(BINARY) -- --file-headers
	cargo size --bin $(BINARY) --release -- -A
	@echo "Flashing into Flash (release build)"
	cargo flash --release --chip "${CHIP_NAME}"

.PHONY: tools
tools: $(TOOLS_BIN)/probe-rs
	rustup update
	rustup component add rustfmt
	rustup component add clippy

$(TOOLS_BIN)/%:
	cargo install --root target/tools/cargo "$(@F)"

$(TOOLS_BIN)/probe-rs: $(TOOLS_BIN)/cargo-binutils $(TOOLS_BIN)/cargo-expand
	rustup component add llvm-tools-preview
	rustup target add $(TARGET)
	cargo install --root target/tools/cargo "$(@F)" --features cli

# probe-target/STM32F4_Series.yaml
probe-target/%_Series.yaml: $(TOOLS_BIN)/target-gen
	@mkdir -p $(@D)
	$(TOOLS_BIN)/target-gen arm -f "$*" "$(@D)"

.PHONY: clean
clean:
	cargo clean
	rm -rf $(TOOLS_BIN) target/
