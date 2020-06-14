use std::fmt;

use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;
pub const OPCODE_SIZE: u16 = 2;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

pub struct Cpu {
    v: [u8; 16],
    pc: u16,
    i: u16,
}

// Related Function
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            pc: PROGRAM_START,
            i: 0,
        }
    }
}

// Methods
impl Cpu {
    pub fn write_reg_v(&mut self, index: u8, value: u8) {
        self.v[index as usize] = value;
    }
    pub fn read_reg_v(&mut self, index: u8) -> u8{
        self.v[index as usize]
    }

    
    fn draw_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        println!("Drawing at ({}, {}) with height = {}", x, y, height);
        let set_vf = false;
        for j in 0..height {
            let b = ram.read_byte(self.i + (j as u16));
            for k in 0..8 {
                let bit = (b >> (7 - k)) & 0b0000_0001;
                match bit {
                    0 => print!("-"),
                    1 => print!("#"),
                    _ => unreachable!()
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

// Opcode Methods
impl Cpu {
    pub fn run_opcode(&mut self, ram: &mut Ram) {
        // Get Opcode
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let opcode = (hi << 8) | lo;

        // Parse info from Opcode
        let nnn = (opcode & 0x0FFF) as u16;
        let nn = (opcode & 0x00FF) as u8; // also called "kk" or "lo" or "byte"
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let nibbles = (
            ((opcode & 0xF000) >> 12) as u8,
            ((opcode & 0x0F00) >> 8) as u8,
            ((opcode & 0x00F0) >> 4) as u8,
            (opcode & 0x000F) as u8,
        );

        println!("opcode: {:#x} {:?}", opcode, nibbles);

        let pc_change: ProgramCounter = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS (Display)
                // Clear the display.
                ProgramCounter::Next
            },

            (0x0, 0x0, 0xE, 0xE) => {
                // RET (Flow)
                // Return from a subroutine.
                ProgramCounter::Next
            },

            (0x1, _, _, _) => {
                // JP addr (Flow)
                // Jump to location nnn.
                ProgramCounter::Next
            },

            (0x2, _, _, _) => {
                // CALL addr (Flow)
                // Call subroutine at nnn.
                ProgramCounter::Next
            },

            (0x3, _, _, _) => {
                // SE Vx, byte (Cond)
                // Skip next instruction if Vx = kk.

                let vx = self.read_reg_v(x);
                if vx == nn {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            },

            (0x4, _, _, _) => {
                // SNE Vx, byte (Cond)
                // Skip next instruction if Vx != kk.
                ProgramCounter::Next
            },

            (0x5, _, _, 0x0) => {
                // SE Vx, Vy (Cond)
                // Skip next instruction if Vx = Vy.
                ProgramCounter::Next
            },

            (0x6, _, _, _) => {
                // LD Vx, byte (Const)
                // Set Vx = kk.

                self.write_reg_v(x, nn);
                ProgramCounter::Next
            },

            (0x7, _, _, _) => {
                // ADD Vx, byte (Const)
                // Set Vx = Vx + kk.

                let vx = self.read_reg_v(x);
                self.write_reg_v(x, vx.wrapping_add(nn));
                ProgramCounter::Next
            },

            (0x8, _, _, 0x0) => {
                // LD Vx, Vy (Assign)
                // Set Vx = Vy.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x1) => {
                // OR Vx, Vy (BitOp)
                // Set Vx = Vx OR Vy.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x2) => {
                // AND Vx, Vy (BitOp)
                // Set Vx = Vx AND Vy.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy (BitOp)
                // Set Vx = Vx XOR Vy.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy (Math)
                // Set Vx = Vx + Vy, set VF = carry.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x5) => {
                // SUB Vx, Vy (Math)
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x6) => {
                // SHR Vx {, Vy} (BitOp)
                // Set Vx = Vx SHR 1.
                ProgramCounter::Next
            },

            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy (Math)
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                ProgramCounter::Next
            },

            (0x8, _, _, 0xE) => {
                // SHL Vx {, Vy} (BitOp)
                // Set Vx = Vx SHL 1.
                ProgramCounter::Next
            },

            (0x9, _, _, 0x0) => {
                // SNE Vx, Vy (Cond)
                // Skip next instruction if Vx != Vy.
                ProgramCounter::Next
            },

            (0xA, _, _, _) => {
                // LD I, addr (MEM)
                // Set I = nnn.
                ProgramCounter::Next
            },

            (0xB, _, _, _) => {
                // JP V0, addr (Flow)
                // Jump to location nnn + V0.
                ProgramCounter::Next
            },

            (0xC, _, _, _) => {
                // RND Vx, byte (Rand)
                // Set Vx = random byte AND kk.
                ProgramCounter::Next
            },

            (0xD, _, _, _) => {
                // DRW Vx, Vy, nibble (Disp)
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                println!("{} {}", x, y);
                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                self.draw_sprite(ram, vx, vy, n);
                ProgramCounter::Next
            },

            (0xE, _, 0x9, 0xE) => {
                // SKP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is pressed.
                ProgramCounter::Next
            },

            (0xE, _, 0xA, 0x1) => {
                // SKNP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is not pressed.
                ProgramCounter::Next
            },

            (0xF, _, 0x0, 0x7) => {
                // LD Vx, DT (Timer)
                // Set Vx = delay timer value.
                ProgramCounter::Next
            },

            (0xF, _, 0x0, 0xA) => {
                // LD Vx, K (KeyOp)
                // Wait for a key press, store the value of the key in Vx.
                ProgramCounter::Next
            },

            (0xF, _, 0x1, 0x5) => {
                // LD DT, Vx (Timer)
                // Set delay timer = Vx.
                ProgramCounter::Next
            },

            (0xF, _, 0x1, 0x8) => {
                // LD ST, Vx (Sound)
                // Set sound timer = Vx.
                ProgramCounter::Next
            },

            (0xF, _, 0x1, 0xE) => {
                // ADD I, Vx (MEM)
                // Set I = I + Vx.

                let vx = self.read_reg_v(x);
                self.i += vx as u16;
                
                ProgramCounter::Next
            },

            (0xF, _, 0x2, 0x9) => {
                // LD F, Vx (MEM)
                // Set I = location of sprite for digit Vx.
                ProgramCounter::Next
            },

            (0xF, _, 0x3, 0x3) => {
                // LD B, Vx (BCD)
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                ProgramCounter::Next
            },

            (0xF, _, 0x5, 0x5) => {
                // LD [I], Vx (MEM)
                // Store registers V0 through Vx in memory starting at location I.
                ProgramCounter::Next
            },

            (0xF, _, 0x6, 0x5) => {
                // LD Vx, [I] (MEM)
                // Read registers V0 through Vx from memory starting at location I.
                ProgramCounter::Next
            },
            
            _ => {
                println!("{:?}", self);
                panic!("Unrecognized Instruction")
        
            }
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr
        }
    }
}

// Testing
impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pc: {:#X}\n", self.pc)?;
        write!(f, "v: ")?;
        write!(f, "[")?;
        for i in 0..self.v.len() {
            write!(f, "{:#X}", self.v[i])?;
            if i < self.v.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]\n")?;
        write!(f, "i: {:#X}", self.i)
    }
}