MAKEFLAGS += --no-print-directory

IMG ?= target/img_hello_world_debug.bin

.PHONY: FORCE all clean opensbi user-programs debug-imgs release-imgs debug
all: opensbi debug-imgs release-imgs

clean:
	@$(MAKE) -s -C os clean
	@$(MAKE) -s -C user clean
	@$(MAKE) -s -C third_party/opensbi clean
	@rm -rf target

opensbi:
	@$(MAKE) -s -C third_party/opensbi LLVM=1 PLATFORM=generic

user-programs:
	@$(MAKE) -s -C user

debug-imgs:	user-programs
	@$(MAKE) -s $(patsubst user/target/%_debug.bin, target/img_%_debug.bin, $(wildcard user/target/*_debug.bin))

release-imgs: user-programs
	@$(MAKE) -s $(patsubst user/target/%_release.bin, target/img_%_release.bin, $(wildcard user/target/*_release.bin))

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

target/os_debug.bin: FORCE
	@$(MAKE) -s -C os bin-debug
	@mkdir -p target
	@cp os/target/os_debug.bin target/os_debug.bin

target/os_release.bin: FORCE
	@$(MAKE) -s -C os bin-release
	@mkdir -p target
	@cp os/target/os_release.bin target/os_release.bin

target/img_%_debug.bin: target/os_debug.bin user/target/%_debug.bin
	@echo "Creating $@"
	@cp target/os_debug.bin $@
	@truncate $@ --size 2MiB
	@cat user/target/$*_debug.bin >> $@

target/img_%_release.bin: target/os_release.bin user/target/%_release.bin
	@echo "Creating $@"
	@cp target/os_release.bin $@
	@truncate $@ --size 2MiB
	@cat user/target/$*_release.bin >> $@
