
# all :


# configure  :
# 	rustup toolchain install nightly --allow-downgrade
# 	rustup target add thumbv7em-none-eabihf
# 	rustup component add llvm-tools-preview
# 	cargo install bootimage
# 	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# install : 

bootimage : 
	cargo bootimage

qemu : bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x64-kfs_1/debug/bootimage-kfs-1.bin

asm_files : 
	mkdir -p obj
	nasm -f elf64 asm/boot.asm -o obj/boot.o
	nasm -f elf64 asm/multiboot_header.asm -o obj/multiboot_header.o

kernel_basic : asm_files
	ld -n -o obj/kernel.bin -T asm/linker.ld obj/multiboot_header.o obj/boot.o

iso_basic : kernel_basic
	cp obj/kernel.bin isofiles/boot/kernel.bin
	grub-mkrescue -d /usr/lib/grub/i386-pc -o os.iso isofiles

boot_basic : iso_basic
	qemu-system-x86_64 -cdrom os.iso