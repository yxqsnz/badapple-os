use core::fmt::Write;

use uefi::{
    prelude::*,
    proto::console::gop::{BltOp, BltPixel, GraphicsOutput},
    table::boot::{OpenProtocolAttributes, OpenProtocolParams},
};

use crate::assets::ASSETS;

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


fn print_frames(gop: &mut GraphicsOutput) {
    let mut x = 0;
    let mut y = 0;

    for asset in ASSETS {
        for line in asset.lines() {
            for character in line.chars() {
                let pixel = match character {
                    '@' => BltPixel::new(0, 0, 0),
                    _ => BltPixel::new(255, 255, 255)
                };

                let operation = BltOp::VideoFill { color: pixel, dest: (x, y), dims: (12, 12) };
                gop.blt(operation).ok();
                x += 5;
            }

            y += 7;
            x = 0;
        }
    }
}

pub fn ekern(image: Handle, st: &mut SystemTable<Boot>) {
    let mut io = unsafe { st.unsafe_clone() };
    let stdout = io.stdout();
    let _ = writeln!(stdout, "SysInfo: Starting Graphic protocol.");
    let bt = st.boot_services();

    if let Ok(handle) = bt.get_handle_for_protocol::<GraphicsOutput>() {
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
            print_frames(gop);

            let _ = writeln!(stdout, "Finished");
            let _ = writeln!(stdout, "OS by Yxqsnz (https://github.com/yxqsnz) ");
            let _ = writeln!(stdout, "Assets by S0ra (https://github.com/S0raWasTaken)");
            let _ = writeln!(stdout, "Why? I don't know.");
        }
    }
}
