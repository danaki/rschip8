use rand::Rng;

use crate::Video;
use crate::Op;
use std::convert::TryFrom;

pub const FONTSET_BASE: usize = 0x050;
pub const PROGRAM_BASE: usize = 0x200;

const FONTSET: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub struct Cpu {
    pub video: Video,
    ram: Vec<u8>,
    v: [u8; 16],
    i: u16,
    pc: usize,
    stack: [usize; 16],
    sp: usize,
    dt: u8,
    key: u8,
    wait_key: usize
}


impl Cpu {
    pub fn new(program: &[u8]) -> Self {
        let mut cpu = Cpu {
            ram: vec![0u8; 0xfff],
            v: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            i: 0,
            pc: 0,
            stack: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            sp: 0,
            dt: 0,
            video: Video::new(),
            key: 0xff,
            wait_key: 0xff
        };

        cpu.load(&FONTSET, FONTSET_BASE);
        cpu.load(&program, PROGRAM_BASE);
        cpu.pc = PROGRAM_BASE;

        cpu
    }

    pub fn current(&mut self) -> (usize, u16, Op) {
        let word = u16::from(self.ram[self.pc]) << 8 | u16::from(self.ram[self.pc + 1]);
        (
            self.pc,
            word,
            Cpu::parse(word)
        )
    }

    #[allow(clippy::too_many_lines)]
    pub fn step(&mut self, op: &Op) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        match *op {
            Op::LD_Vx_byte { x, nn } => {
                self.v[x] = nn;
                self.pc += 2;
            },
            Op::ADD_Vx_byte { x, nn } => {
                self.v[x] = self.v[x].wrapping_add(nn);
                self.pc += 2;
            },
            Op::LD_Vx_Vy { x, y } => {
                self.v[x] = self.v[y];
                self.pc += 2;
            },
            Op::ADD_Vx_Vy { x, y } => {
                match self.v[x].checked_add(self.v[y]) {
                    Some(_v) => {
                        self.v[0xf] = 0;
                    }
                    None => {
                        self.v[0xf] = 1;
                    }
                };

                self.v[x] = self.v[x].wrapping_add(self.v[y]);
                self.pc += 2;
            },
            Op::SUB_Vx_Vy { x, y } => {
                match self.v[x].checked_sub(self.v[y]) {
                    Some(_v) => {
                        self.v[0xf] = 1;
                    }
                    None => {
                        self.v[0xf] = 0;
                    }
                };

                self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                self.pc += 2;
            },
            Op::SUBN_Vx_Vy { x, y } => {
                match self.v[y].checked_sub(self.v[x]) {
                    Some(_v) => {
                        self.v[0xf] = 1;
                    }
                    None => {
                        self.v[0xf] = 0;
                    }
                };

                self.v[y] = self.v[y].wrapping_sub(self.v[x]);
                self.pc += 2;
            },
            Op::AND_Vx_Vy { x, y } => {
                self.v[x] = self.v[x] & self.v[y];
                self.pc += 2;
            },
            Op::OR_Vx_Vy { x, y } => {
                self.v[x] = self.v[x] | self.v[y];
                self.pc += 2;
            },
            Op::XOR_Vx_Vy { x, y } => {
                self.v[x] = self.v[x] ^ self.v[y];
                self.pc += 2;
            },
            Op::SHR_Vx_Vy { x, y } => {
                self.v[0xf] = self.v[y] & 0x01;
                self.v[x] = self.v[y] >> 1;
                self.pc += 2;
            },
            Op::SHL_Vx_Vy { x, y } => {
                self.v[0xf] = (self.v[y] & 0x80) >> 7;
                self.v[x] = self.v[y] << 1;
                self.pc += 2;
            },
            Op::RND_Vx_byte { x, nn } => {
                let rnd: u8 = rand::thread_rng().gen();
                self.v[x] = rnd & nn;
                self.pc += 2;
            },
            Op::JP_addr { nnn } => {
                self.pc = nnn;
            },
            Op::JP_V0_addr { nnn } => {
                self.pc = nnn + self.v[0] as usize;
            },
            Op::CALL_addr { nnn } => {
                self.stack[self.sp] = self.pc + 2;
                self.sp += 1;
                self.pc = nnn;
            },
            Op::RET {} => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            },
            Op::SE_Vx_byte { x, nn } => {
                self.pc += if self.v[x] == nn { 4 } else { 2 }
            },
            Op::SE_Vx_Vy { x, y } => {
                self.pc += if self.v[x] == self.v[y] { 4 } else { 2 }
            },
            Op::SNE_Vx_byte { x, nn } => {
                self.pc += if self.v[x] == nn { 2 } else { 4 }
            },
            Op::SNE_Vx_Vy { x, y } => {
                self.pc += if self.v[x] == self.v[y] { 2 } else { 4 }
            },
            Op::LD_DT_Vx { x } => {
                self.dt = self.v[x];
                self.pc += 2;
            },
            Op::LD_Vx_DT { x } => {
                self.v[x] = self.dt;
                self.pc += 2;
            },
            Op::LD_ST_Vx { x: _ } => {
                self.pc += 2;
            },
            Op::LD_Vx_K { x } => {
                self.wait_key = x;
                self.pc += 2;
            },
            Op::SKP_Vx { x } => {
                self.pc += if self.v[x] == self.key { 4 } else { 2 }
            },
            Op::SKNP_Vx { x } => {
                self.pc += if self.v[x] == self.key { 2 } else { 4 }
            },
            Op::LD_I_addr { nnn } => {
                self.i = u16::try_from(nnn).unwrap();
                self.pc += 2;
            },
            Op::ADD_I_Vx { x } => {
                match self.i.checked_add(u16::from(self.v[x])) {
                    Some(_v) => {
                        self.v[0xf] = 0;
                    }
                    None => {
                        self.v[0xf] = 1;
                    }
                };

                self.i = self.i.wrapping_add(u16::from(self.v[x]));
                self.pc += 2;
            },
            Op::DRW_Vx_Vy_nibble { x, y, n } => {
                self.v[0xf] = self.video.draw(
                    &self.ram[(self.i as usize)..((self.i + u16::from(n)) as usize)],
                    i32::from(self.v[x]),
                    i32::from(self.v[y])
                ) as u8;
                self.pc += 2;
            },
            Op::CLS {} => {
                self.video.clear();
                self.pc += 2;
            },
            Op::LD_F_Vx { x } => {
                self.i = u16::try_from(FONTSET_BASE + usize::from(self.v[x]) * 5).unwrap();
                self.pc += 2;
            },
            Op::LD_B_Vx { x } => {
                let mut val = self.v[x];
                for i in (0..3).rev() {
                    self.ram[self.i as usize + i] = val % 10;
                    val /= 10;
                }
                self.pc += 2;
            },
            Op::LD_I_Vx { x } => {
                for i in 0..x {
                    self.ram[self.i as usize + i] = self.v[i];
                }
                self.pc += 2;
            },
            Op::LD_Vx_I { x } => {
                for i in 0..x {
                    self.v[i] = self.ram[self.i as usize + i];
                }
                self.pc += 2;
            },
            Op::UNKNOWN {} => {
                panic!("UNKNOWN op")
            },
        }
    }

    pub fn disassemble(data: &[u8]) {
        let list = data.chunks(2)
            .map(|x| u16::from(x[0]) << 8 | u16::from(x[1]))
            .collect::<Vec<_>>();

        for (i, word) in list.iter().enumerate() {
            println!("{:#06x} {:#06x} {}", PROGRAM_BASE + i * 2, word, Cpu::parse(*word));
        }
    }

    pub fn waiting_key(&mut self) -> bool {
        self.wait_key != 0xff
    }

    pub fn press(&mut self, key: u8) {
        if self.wait_key != 0xff {
            self.v[self.wait_key] = key;
            self.wait_key = 0xff;
        }

        self.key = key;
    }

    pub fn release(&mut self) {
        self.key = 0xff;
    }

    fn load(&mut self, data: &[u8], base: usize) {
        self.ram[base..base + (data.len() as usize)].copy_from_slice(&data);
    }

    fn parse(op: u16) -> Op {
        let x =  ((op & 0x0f00) >> 8) as usize;
        let y =  ((op & 0x00f0) >> 4) as usize;
        let n =   (op & 0x000f) as u8;
        let nn =  (op & 0x00ff) as u8;
        let nnn = (op & 0x0fff) as usize;

        if op & 0xf000 == 0x6000 {
            Op::LD_Vx_byte { x, nn }
        } else if op & 0xf000 == 0x7000 {
            Op::ADD_Vx_byte { x, nn }
        } else if op & 0xf00f == 0x8000 {
            Op::LD_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8004 {
            Op::ADD_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8005 {
            Op::SUB_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8007 {
            Op::SUBN_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8002 {
            Op::AND_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8001 {
            Op::OR_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8003 {
            Op::XOR_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x8006 {
            Op::SHR_Vx_Vy { x, y }
        } else if op & 0xf00f == 0x800e {
            Op::SHL_Vx_Vy { x, y }
        } else if op & 0xf000 == 0xc000 {
            Op::RND_Vx_byte { x, nn }
        } else if op & 0xf000 == 0x1000 {
            Op::JP_addr { nnn }
        } else if op & 0xf000 == 0xb000 {
            Op::JP_V0_addr { nnn }
        } else if op & 0xf000 == 0x2000 {
            Op::CALL_addr { nnn }
        } else if op == 0x00ee {
            Op::RET {}
        } else if op & 0xf000 == 0x3000 {
            Op::SE_Vx_byte { x, nn }
        } else if op & 0xf00f == 0x5000 {
            Op::SE_Vx_Vy { x, y }
        } else if op & 0xf000 == 0x4000 {
            Op::SNE_Vx_byte { x, nn }
        } else if op & 0xf00f == 0x9000 {
            Op::SNE_Vx_Vy { x, y }
        } else if op & 0xf0ff == 0xf015 {
            Op::LD_DT_Vx { x }
        } else if op & 0xf0ff == 0xf007 {
            Op::LD_Vx_DT { x }
        } else if op & 0xf0ff == 0xf018 {
            Op::LD_ST_Vx { x }
        } else if op & 0xf0ff == 0xf00a {
            Op::LD_Vx_K { x }
        } else if op & 0xf0ff == 0xe09e {
            Op::SKP_Vx { x }
        } else if op & 0xf0ff == 0xe0a1 {
            Op::SKNP_Vx { x }
        } else if op & 0xf000 == 0xa000 {
            Op::LD_I_addr { nnn }
        } else if op & 0xf0ff == 0xf01e {
            Op::ADD_I_Vx { x }
        } else if op & 0xf000 == 0xd000 {
            Op::DRW_Vx_Vy_nibble { x, y, n }
        } else if op & 0xffff == 0x00e0 {
            Op::CLS {}
        } else if op & 0xf0ff == 0xf029 {
            Op::LD_F_Vx { x }
        } else if op & 0xf0ff == 0xf033 {
            Op::LD_B_Vx { x }
        } else if op & 0xf0ff == 0xf055 {
            Op::LD_I_Vx { x }
        } else if op & 0xf0ff == 0xf065 {
            Op::LD_Vx_I { x }
        } else {
            Op::UNKNOWN {}
        }
    }
}
