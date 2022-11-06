#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use core::{fmt::Write, panic::PanicInfo};
use uefi::{
    prelude::*,
    proto::console::pointer::Pointer,
    table::boot::{LoadImageSource, SearchType},
};

static mut SYSTEM_TABLE: Option<SystemTable<Boot>> = None;

#[entry]
fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> Status {
    unsafe {
        SYSTEM_TABLE = Some(system_table.unsafe_clone());
    }
    st().stdout().write_str("Starting...\n").unwrap();

    let driver = bt()
        .load_image(
            image,
            LoadImageSource::FromBuffer {
                buffer: include_bytes!("../UsbMouseDxe.efi"),
                file_path: None,
            },
        )
        .unwrap();
    bt().start_image(driver).unwrap();
    let all_handles = bt().locate_handle_buffer(SearchType::AllHandles).unwrap();
    for handle in all_handles.handles() {
        // Ignore errors; not all handles will have a new driver to
        // connect.
        let _ = bt().connect_controller(*handle, None, None, true);
    }

    let handle = bt().get_handle_for_protocol::<Pointer>().unwrap();
    let mut pointer = bt().open_protocol_exclusive::<Pointer>(handle).unwrap();

    st().stdout()
        .write_fmt(core::format_args!("Mode: {:#?}\n", pointer.mode()))
        .unwrap();
    loop {
        if let Ok(Some(state)) = pointer.read_state() {
            st().stdout()
                .write_fmt(core::format_args!("{state:#?}\n"))
                .unwrap();
        }
    }
}

fn st() -> &'static mut SystemTable<Boot> {
    unsafe {
        let table_ref = SYSTEM_TABLE.as_ref().unwrap();
        core::ptr::NonNull::new(table_ref as *const _ as *mut _)
            .unwrap()
            .as_mut()
    }
}

fn bt() -> &'static BootServices {
    st().boot_services()
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    st().stdout()
        .write_fmt(core::format_args!("{info:#?}"))
        .unwrap();
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn out_of_memory(layout: core::alloc::Layout) -> ! {
    panic!("Ran out of free memory while trying to allocate {layout:#?}");
}
