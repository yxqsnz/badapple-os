#![no_main]
#![no_std]
#![feature(abi_efiapi)]


mod assets;
mod kernel;

use kernel::ekern;
use uefi::prelude::*;
extern crate alloc;

#[entry]
fn main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    ekern(image, &mut system_table);

    loop {}
}
