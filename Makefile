CHIP_NAME?=STM32H743VGTx

default: lint test build

lint:
	cargo fmt

test:
	cargo test

build:
	cargo build --features=stm32h743v,rt

run: flash

.PHONY: flash
flash: probe-target/STM32F4_Series.yaml probe-target/STM32F4_Series.yaml
	cargo flash --release --chip ${CHIP_NAME}

.PHONY: tools
tools: init
	rustup update
	rustup component add llvm-tools-preview
	rustup target add thumbv7em-none-eabihf
	cargo install probe-rs --features cli

tools/cargo/%:
	cargo install --root tools/cargo $(@F)

probe-target/STM32F4_Series.yaml: tools/cargo/bin/target-gen
	@mkdir -p $(@D)
	cargo install probe-rs target-gen
	./tools/cargo/bin/target-gen arm -f STM32F4 $(@D)

probe-target/STM32G4_Series.yaml: tools/cargo/bin/target-gen
	./tools/cargo/bin/target-gen arm -f STM32G4 $(@D)

.PHONY: clean
clean:
	cargo clean
	rm -rf tools/cargo probe-target/*.yaml

