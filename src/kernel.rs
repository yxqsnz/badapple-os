use alloc::{string::String, vec::Vec};
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

fn pixel(character: char) -> BltPixel {
    match character {
        '@' => BltPixel::new(255, 255, 255),
        '+' => BltPixel::new(131, 131, 131),
        '=' => BltPixel::new(101, 101, 101),
        '-' => BltPixel::new(81, 81, 81),
        ':' => BltPixel::new(41, 41, 41),
        '.' => BltPixel::new(20, 20, 20),

        _ => BltPixel::new(0, 0, 0),
    }
}

fn print_frames(bt: &BootServices, gop: &mut GraphicsOutput, assets: Vec<String>) {
    let mut x = 0;
    let mut y = 0;

    for asset in assets {
        for line in asset.lines() {
            for character in line.chars() {
                let pixel = pixel(character);

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
    let bt = st.boot_services();
    println!("EKern: Preparing Graphical output...");

    let Ok(handle) = bt.get_handle_for_protocol::<GraphicsOutput>() else {
        println!("Error: Can't find any Graphical Output Protocol for this device");
        panic!("Halting");
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

        println!("Badapple os - powered by Rust & UEFI");

        let assets = Assets::open(&bt);

        println!("Take off your sets..");
        let frames = assets.collect();

        println!("GO!");
        set_graphics_mode(gop);
        print_frames(&bt, gop, frames);

        println!("Finished (by: https://github.com/yxqsnz)");
    }
}
