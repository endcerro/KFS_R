
all : boot_basic

# configure  :
# 	rustup toolchain install nightly --allow-downgrade
# 	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

kvm : iso_basic
	kvm -name kfs -m 1024 -cdrom ./os.iso -boot c
qemu : iso_basic
	qemu-system-x86_64 -cdrom os.iso

rust_files :
	cargo build

asm_files :
	mkdir -p obj
	nasm -f elf64 asm/boot.asm -o obj/boot.o
	nasm -f elf64 asm/multiboot_header.asm -o obj/multiboot_header.o
	nasm -f elf64 asm/long_mode_init.asm -o obj/long_mode_init.o

kernel_basic : asm_files rust_files
	@ld -n -o obj/kernel.bin -T asm/linker.ld obj/multiboot_header.o obj/boot.o obj/long_mode_init.o target/x64-kfs_1/debug/libkfs_1.a

iso_basic : kernel_basic
	mkdir -p isofiles
	mkdir -p isofiles/boot
	mkdir -p isofiles/boot/grub
	cp src/grub.cfg isofiles/boot/grub
	cp obj/kernel.bin isofiles/boot/kernel.bin
	grub-mkrescue -d /usr/lib/grub/i386-pc -o os.iso isofiles

boot_basic : kvm