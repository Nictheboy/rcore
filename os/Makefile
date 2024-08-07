OBJCOPY=rust-objcopy
TARGET=riscv64gc-unknown-none-elf
PROJECT=os

ELF_DEBUG=target/$(TARGET)/debug/$(PROJECT)
ELF_RELEASE=target/$(TARGET)/release/$(PROJECT)
BIN_DEBUG=target/$(PROJECT)_debug.bin
BIN_RELEASE=target/$(PROJECT)_release.bin

.PHONY: all clean elf-debug elf-release bin-debug bin-release FORCE
all: bin-debug bin-release

clean:
	@cargo clean

elf-debug: $(ELF_DEBUG)
elf-release: $(ELF_RELEASE)
bin-debug: $(BIN_DEBUG)
bin-release: $(BIN_RELEASE)

$(ELF_DEBUG): FORCE
	@cargo build
$(ELF_RELEASE): FORCE
	@cargo build --release
$(BIN_DEBUG): $(ELF_DEBUG)
	@$(OBJCOPY) --strip-all $(ELF_DEBUG) -O binary $(BIN_DEBUG)
$(BIN_RELEASE): $(ELF_RELEASE)
	@$(OBJCOPY) --strip-all $(ELF_RELEASE) -O binary $(BIN_RELEASE)
