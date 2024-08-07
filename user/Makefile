TASKS=hello_world-task

.PHONY: all clean $(TASKS:task=clean) \
	$(TASKS:task=debug-elf) $(TASKS:task=release-elf) \
	$(TASKS:task=debug-bin) $(TASKS:task=release-bin)

all: $(TASKS:task=debug-bin) $(TASKS:task=release-bin)

clean: $(TASKS:task=clean)
	@rm -rf target

$(TASKS:task=clean):
	@cd $(@:-clean=) && cargo clean

$(TASKS:task=debug-elf):
	@cd $(@:-debug-elf=) && cargo build

$(TASKS:task=release-elf):
	@cd $(@:-release-elf=) && cargo build --release

$(TASKS:task=debug-bin): %-debug-bin: %-debug-elf
	@mkdir -p target
	@rust-objcopy --strip-all \
		$(@:-debug-bin=)/target/riscv64gc-unknown-none-elf/debug/$(@:-debug-bin=) \
		-O binary \
		target/$(@:-debug-bin=_debug).bin

$(TASKS:task=release-bin): %-release-bin: %-release-elf
	@mkdir -p target
	@rust-objcopy --strip-all \
		$(@:-release-bin=)/target/riscv64gc-unknown-none-elf/release/$(@:-release-bin=) \
		-O binary \
		target/$(@:-release-bin=_release).bin
