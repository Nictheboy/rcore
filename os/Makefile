OBJCOPY=rust-objcopy
TARGET=riscv64gc-unknown-none-elf
PROJECT=os

ELF_DEBUG=target/$(TARGET)/debug/$(PROJECT)
ELF_RELEASE=target/$(TARGET)/release/$(PROJECT)
BIN_DEBUG=target/$(TARGET)/debug/$(PROJECT).bin
BIN_RELEASE=target/$(TARGET)/release/$(PROJECT).bin

all: bin-debug bin-release

clean:
	@cargo clean

elf-debug:
	@cargo build

elf-release:
	@cargo build --release

bin-debug: $(BIN_DEBUG)

bin-release: $(BIN_RELEASE)

$(ELF_DEBUG): elf-debug
$(ELF_RELEASE): elf-release
$(BIN_DEBUG): elf-debug
	$(OBJCOPY) --strip-all target/$(TARGET)/debug/$(PROJECT) -O binary target/$(TARGET)/debug/$(PROJECT).bin
$(BIN_RELEASE): elf-release
	$(OBJCOPY) --strip-all target/$(TARGET)/release/$(PROJECT) -O binary target/$(TARGET)/release/$(PROJECT).bin
