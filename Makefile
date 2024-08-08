MAKEFLAGS += --no-print-directory

IMG ?= target/hello_world_debug.bin

.PHONY: FORCE all clean opensbi user-programs debug-imgs release-imgs debug
all: opensbi debug-imgs release-imgs

clean:
	@$(MAKE) -s -C kernel clean
	@$(MAKE) -s -C user clean
	@$(MAKE) -s -C third_party/opensbi clean
	@rm -rf target

opensbi:
	@$(MAKE) -s -C third_party/opensbi LLVM=1 PLATFORM=generic

user-programs:
	@$(MAKE) -s -C user

debug-imgs:	user-programs
	@$(MAKE) -s $(patsubst user/target/%_debug.bin, target/%_debug.bin, $(wildcard user/target/*_debug.bin))

release-imgs: user-programs
	@$(MAKE) -s $(patsubst user/target/%_release.bin, target/%_release.bin, $(wildcard user/target/*_release.bin))

run: all
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios third_party/opensbi/build/platform/generic/firmware/fw_payload.bin \
		-device loader,file=$(IMG),addr=0x80200000

debug: all
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios third_party/opensbi/build/platform/generic/firmware/fw_payload.bin \
		-device loader,file=$(IMG),addr=0x80200000 \
		-s -S \
		&
	@gdb \
		-ex "file os/target/riscv64gc-unknown-none-elf/debug/os" \
		-ex 'set arch riscv:rv64' \
		-ex 'b *0x80200000' \
		-ex 'target remote localhost:1234' \
		-ex 'c'
	@kill $$(lsof -t -i:1234)

target/kernel_debug.bin: FORCE
	@$(MAKE) -s -C kernel bin-debug
	@mkdir -p target
	@cp kernel/target/kernel_debug.bin target/kernel_debug.bin

target/kernel_release.bin: FORCE
	@$(MAKE) -s -C kernel bin-release
	@mkdir -p target
	@cp kernel/target/kernel_release.bin target/kernel_release.bin

target/%_debug.bin: target/kernel_debug.bin user/target/%_debug.bin
	@echo "Creating $@"
	@cp target/kernel_debug.bin $@
	@truncate $@ --size 2MiB
	@cat user/target/$*_debug.bin >> $@

target/%_release.bin: target/kernel_release.bin user/target/%_release.bin
	@echo "Creating $@"
	@cp target/kernel_release.bin $@
	@truncate $@ --size 2MiB
	@cat user/target/$*_release.bin >> $@
