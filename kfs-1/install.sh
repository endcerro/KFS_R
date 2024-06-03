#! /bin/bash


function binutils () 
{
    cd
    export PREFIX="$HOME/cross"
    export TARGET=i686-elf
    export PATH="$PREFIX/bin:$PATH"
    git clone git://sourceware.org/git/binutils-gdb.git cross
    cd 
    mkdir build-binutils
    cd build-binutils
    ../cross/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
    make
    make install
    cd - 
}

function rust ()
{
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}
function deps ()
{
    sudo apt install -y build-essential make bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo libisl-dev nasm grub-common xorriso sudo grub-pc-bin
}

binutils