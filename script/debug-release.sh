#!/bin/sh

qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios $(dirname $0)/../bootloader/rustsbi-qemu.bin \
    -device loader,file=$(dirname $0)/../os/target/riscv64gc-unknown-none-elf/release/os,addr=0x80200000 \
    -s -S \
    &
QEMU_PID=$!

gdb \
    -ex "file $(dirname $0)/../os/target/riscv64gc-unknown-none-elf/release/os" \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234' \
    -ex 'b *0x80200000' \
    -ex 'c'

kill $QEMU_PID
