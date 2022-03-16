
#[allow(non_camel_case_types)]
pub enum Op {
    LD_Vx_byte {
        // 6xkk
        x: usize,
        nn: u8
    },
    ADD_Vx_byte {
        // 7xkk
        x: usize,
        nn: u8
    },
    LD_Vx_Vy {
        // 8xy0
        x: usize,
        y: usize
    },
    ADD_Vx_Vy {
        // 8xy4
        x: usize,
        y: usize
    },
    SUB_Vx_Vy {
        // 8xy5
        x: usize,
        y: usize
    },
    SUBN_Vx_Vy {
        // 8xy7
        x: usize,
        y: usize
    },
    AND_Vx_Vy {
        // 8xy2
        x: usize,
        y: usize
    },
    OR_Vx_Vy {
        // 8xy1
        x: usize,
        y: usize
    },
    XOR_Vx_Vy {
        // 8xy3
        x: usize,
        y: usize
    },
    SHR_Vx_Vy {
        // 8xy6
        x: usize,
        y: usize
    },
    SHL_Vx_Vy {
        // 8xyE
        x: usize,
        y: usize
    },
    RND_Vx_byte {
        // Cxkk
        x: usize,
        nn: u8
    },
    JP_addr {
        // 1nnn
        nnn: usize
    },
    JP_V0_addr {
        // Bnnn
        nnn: usize
    },
    CALL_addr {
        // 2nnn
        nnn: usize
    },
    RET {
        // 00EE
    },
    SE_Vx_byte {
        // 3xkk
        x: usize,
        nn: u8
    },
    SE_Vx_Vy {
        // 5xy0
        x: usize,
        y: usize
    },
    SNE_Vx_byte {
        // 4xkk
        x: usize,
        nn: u8
    },
    SNE_Vx_Vy {
        // 9xy0
        x: usize,
        y: usize
    },
    LD_DT_Vx {
        // Fx15
        x: usize
    },
    LD_Vx_DT {
        // Fx07
        x: usize
    },
    LD_ST_Vx {
        // Fx18
        x: usize
    },
    LD_Vx_K {
        // Fx0A
        x: usize
    },
    SKP_Vx {
        // Ex9E
        x: usize
    },
    SKNP_Vx {
        // ExA1
        x: usize
    },
    LD_I_addr {
        // Annn
        nnn: usize
    },
    ADD_I_Vx {
        // Fx1E
        x: usize
    },
    DRW_Vx_Vy_nibble {
        // Dxyn
        x: usize,
        y: usize,
        n: u8
    },
    CLS {
        // 00E0
    },
    LD_F_Vx {
        // Fx29
        x: usize,
    },
    LD_B_Vx {
        // Fx33
        x: usize,
    },
    LD_I_Vx {
        // Fx55
        x: usize,
    },
    LD_Vx_I {
        // Fx65
        x: usize,
    },
    UNKNOWN {
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::LD_Vx_byte { x, nn }         => write!(f, "LD V{:x}, {:#04x}", x, nn),
            Op::ADD_Vx_byte { x, nn }        => write!(f, "ADD V{:x}, {:#04x}", x, nn),
            Op::LD_Vx_Vy { x, y }            => write!(f, "LD V{:x}, V{:x}", x, y),
            Op::ADD_Vx_Vy { x, y }           => write!(f, "ADD V{:x}, V{:x}", x, y),
            Op::SUB_Vx_Vy { x, y }           => write!(f, "SUB V{:x}, V{:x}", x, y),
            Op::SUBN_Vx_Vy { x, y }          => write!(f, "SUBN V{:x}, V{:x}", x, y),
            Op::AND_Vx_Vy { x, y }           => write!(f, "AND V{:x}, V{:x}", x, y),
            Op::OR_Vx_Vy { x, y }            => write!(f, "OR V{:x}, V{:x}", x, y),
            Op::XOR_Vx_Vy { x, y }           => write!(f, "XOR V{:x}, V{:x}", x, y),
            Op::SHR_Vx_Vy { x, y }           => write!(f, "SHR V{:x}, V{:x}", x, y),
            Op::SHL_Vx_Vy { x, y }           => write!(f, "SHL V{:x}, V{:x}", x, y),
            Op::RND_Vx_byte { x, nn }        => write!(f, "RND V{:x}, {:#04x}", x, nn),
            Op::JP_addr { nnn }              => write!(f, "JP {:#05x}", nnn),
            Op::JP_V0_addr { nnn }           => write!(f, "JP V0, {:#05x}", nnn),
            Op::CALL_addr { nnn }            => write!(f, "CALL {:#05x}", nnn),
            Op::RET {}                       => write!(f, "RET"),
            Op::SE_Vx_byte { x, nn }         => write!(f, "SE V{:x}, {:#04x}", x, nn),
            Op::SE_Vx_Vy { x, y }            => write!(f, "SE V{:x}, V{:x}", x, y),
            Op::SNE_Vx_byte { x, nn }        => write!(f, "SNE V{:x}, {:#04x}", x, nn),
            Op::SNE_Vx_Vy { x, y }           => write!(f, "SNE V{:x}, V{:x}", x, y),
            Op::LD_DT_Vx { x }               => write!(f, "LD DT, V{:x}", x),
            Op::LD_Vx_DT { x }               => write!(f, "LD V{:x}, DT", x),
            Op::LD_ST_Vx { x }               => write!(f, "LD ST, V{:x}", x),
            Op::LD_Vx_K { x }                => write!(f, "LD V{:x}, K", x),
            Op::SKP_Vx { x }                 => write!(f, "SKP V{:x}", x),
            Op::SKNP_Vx { x }                => write!(f, "SKNP V{:x}", x),
            Op::LD_I_addr { nnn }            => write!(f, "LD I, {:#05x}", nnn),
            Op::ADD_I_Vx { x }               => write!(f, "ADD I, V{:x}", x),
            Op::DRW_Vx_Vy_nibble { x, y, n } => write!(f, "DRW V{:x}, V{:x}, {:x}", x, y, n),
            Op::CLS {}                       => write!(f, "CLS"),
            Op::LD_F_Vx { x }                => write!(f, "LD F, V{:x}", x),
            Op::LD_B_Vx { x }                => write!(f, "LD B, V{:x}", x),
            Op::LD_I_Vx { x }                => write!(f, "LD [I], V{:x}", x),
            Op::LD_Vx_I { x }                => write!(f, "LD V{:x}, [I]", x),
            Op::UNKNOWN {}                   => write!(f, "UNKNOWN"),
        }
    }
}