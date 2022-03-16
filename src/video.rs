use crate::prelude::*;

const NUM_PIXELS: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug)]
pub struct Video {
    pub ram: Vec<u8>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && x < SCREEN_WIDTH && y >= 0 && y < SCREEN_HEIGHT
}

impl Video {
    pub fn new() -> Self {
        Self {
            ram: vec![0u8; NUM_PIXELS],
        }
    }

    pub fn draw(&mut self, sprite: &[u8], x: i32, y: i32) -> u8 {
        let mut yy = 0;
        let mut result = 0;
        for byte in sprite {
            for xx in 0..8 {
                if in_bounds(x + xx, y + yy) {
                    let idx = map_idx(x + xx, y + yy);
                    let val = byte >> (7 - xx) & 1;
                    if self.ram[idx] != 0 && val != 0 {
                        result = 1;
                    }

                    self.ram[idx] = self.ram[idx] ^ val;
                }
            }
            yy += 1;
        }

        result
    }

    pub fn clear(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                self.ram[idx] = 0;
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.ram[idx] {
                    0 => {
                        ctx.set(x, y, YELLOW, BLACK,
                            to_cp437('.')
                        );
                    }
                    _ => {
                        ctx.set(x, y, GREEN, BLACK,
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }
}