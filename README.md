install uefi-run

```
cargo install uefi-run
```

then build and run it

```
cargo build --release && uefi-run -b /usr/share/edk2-ovmf/x64/OVMF_CODE.fd target/x86_64-unknown-uefi/release/testpointer.efi -f UsbMouseDxe.efi -- -usb -device usb-mouse
```
