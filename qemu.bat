copy $1 iso\EFI\BOOT\BOOTx64.efi
qemu-system-x86_64 -enable-kvm -bios OVMF.fd -m 128 -nographic -drive format=raw,file=fat:rw:iso
