
all :
	cargo bootimage

qemu : all
	qemu-system-x86_64 -drive format=raw,file=target/x64-kfs_1/debug/bootimage-kfs-1.bin


configure  :
	rustup toolchain install nightly --allow-downgrade
	rustup target add thumbv7em-none-eabihf
	rustup component add llvm-tools-preview
	cargo install bootimage

install : 
