#! /bin/bash

function rust ()
{
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
}
function deps ()
{
    sudo apt install -y build-essential make curl nasm grub-common xorriso grub-pc-bin qemu-system-x86
}

deps
rust