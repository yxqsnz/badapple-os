use core::fmt::Write;

use uefi::{
    prelude::*,
    proto::console::gop::{BltOp, BltPixel, GraphicsOutput},
    table::boot::{OpenProtocolAttributes, OpenProtocolParams},
};
use uefi_services::println;

use crate::assets::Assets;

fn set_graphics_mode(gop: &mut GraphicsOutput) {
    let mode = gop
        .modes()
        .find(|mode| {
            let info = mode.info();
            info.resolution() == (640, 480)
        })
        .unwrap();

    gop.set_mode(&mode).expect("Failed to set graphics mode");
}

fn print_frames(bt: &BootServices, gop: &mut GraphicsOutput, assets: Assets) {
    let mut x = 0;
    let mut y = 0;

    for asset in assets {
        for line in asset.lines() {
            for character in line.chars() {
                let pixel = match character {
                    '@' => BltPixel::new(0, 0, 0),
                    _ => BltPixel::new(255, 255, 255),
                };

                let operation = BltOp::VideoFill {
                    color: pixel,
                    dest: (x, y),
                    dims: (12, 12),
                };
                gop.blt(operation).ok();
                x += 5;
            }

            y += 7;
            x = 0;
        }

        bt.stall((1000 / 30) * 1000);
        x = 0;
        y = 0;
    }
}

pub fn ekern(image: Handle, st: &mut SystemTable<Boot>) {
    let mut io = unsafe { st.unsafe_clone() };
    let stdout = io.stdout();
    let _ = writeln!(stdout, "SysInfo: Starting Graphic protocol.");
    let bt = st.boot_services();

    let Ok(handle) = bt.get_handle_for_protocol::<GraphicsOutput>() else {
        panic!("Can't find GOP for this device");
    };

    unsafe {
        let gop = &mut bt
            .open_protocol::<GraphicsOutput>(
                OpenProtocolParams {
                    handle,
                    agent: image,
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            )
            .expect("failed to open Graphics Output Protocol");
        set_graphics_mode(gop);

        let assets = Assets::open(&bt);
        print_frames(&bt, gop, assets);
        println!(">>= Finished");
    }
}
