EFI_PATH="target/x86_64-unknown-uefi/debug/badapple-os.efi"
export CARGO_TARGET_DIR=target

echo "[=] BUILDING"
cargo build --target x86_64-unknown-uefi || exit

echo "[=] EFI DIRECTORY: ${EFI_PATH}"

echo "@= Creating file structure at vm"
mkdir -p vm/EFI/Boot/
mv ${EFI_PATH} vm/EFI/Boot/Bootx64.efi

echo "@= Running QEMU"
qemu-system-x86_64 -drive format=raw,file=fat:rw:vm/ -machine q35 -no-reboot -enable-kvm -bios /usr/share/OVMF/OVMF_CODE.fd
