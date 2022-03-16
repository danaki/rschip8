#![warn(clippy::pedantic)]

mod video;
mod cpu;
mod op;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 64;
    pub const SCREEN_HEIGHT: i32 = 32;
    pub use crate::video::*;
    pub use crate::cpu::*;
    pub use crate::op::*;
}

use prelude::*;

extern crate rand;

use std::env;
use std::fs;

extern crate log;
use log::{debug};

impl GameState for Cpu {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.video.render(ctx);

        if ! self.waiting_key() {
            let (pc, word, op) = self.current();
            debug!("{:#06x} {:#06x} {}", pc, word, op);
            self.step(&op);
        }

        match ctx.key {
            None => {
                self.release();
            }
            Some(key) => {
                if key == VirtualKeyCode::Q {
                    ctx.quitting = true;
                } else {
                    self.press(key as u8);
                }
            }
        }
    }
}

fn main() -> BError {
    if env::args().len() != 3 {
        panic!()
    }

    let command = env::args().nth(1).ok_or("Command r|a")?;
    let filename = env::args().nth(2).ok_or("Provide path to rom")?;
    let rom = fs::read(filename.clone()).expect("Unable to read file");
    let state = Cpu::new(rom.as_slice());

    match command.as_str() {
        "r" => {
            env_logger::init();

            let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
                .unwrap()
                .with_title(filename)
                .with_fps_cap(60.0)
                .build()?;

            main_loop(context, state)
        },
        "d" => {
            Cpu::disassemble(&rom);
            Ok(())
        },
        _ => panic!("Unknown command")
    }

}
