MAKEFLAGS += --no-print-directory

all: opensbi os-bin-debug os-bin-release

clean:
	@$(MAKE) -C os clean
	@$(MAKE) -C third_party/opensbi clean

os-bin-debug:
	@$(MAKE) -C os bin-debug

os-bin-release:
	@$(MAKE) -C os bin-release

opensbi:
	@$(MAKE) -C third_party/opensbi LLVM=1 PLATFORM=generic

debug: os-bin-debug opensbi
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios third_party/opensbi/build/platform/generic/firmware/fw_payload.bin \
		-device loader,file=os/target/riscv64gc-unknown-none-elf/debug/os,addr=0x80200000 \
		-s -S \
		&
	@gdb \
		-ex "file os/target/riscv64gc-unknown-none-elf/debug/os" \
		-ex 'set arch riscv:rv64' \
		-ex 'b *0x80200000' \
		-ex 'target remote localhost:1234' \
		-ex 'c'
	@kill $$(lsof -t -i:1234)
