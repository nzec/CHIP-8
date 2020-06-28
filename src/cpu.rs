use std::fmt;

use crate::bus::Bus;

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
    ret_stack: Vec<u16>,
}

// Related Functions
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }
}

// Basic Methods
impl Cpu {
    pub fn write_reg_v(&mut self, index: u8, value: u8) {
        self.v[index as usize] = value;
    }
    pub fn read_reg_v(&mut self, index: u8) -> u8 {
        self.v[index as usize]
    }

    fn draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        let mut set_vf = false;

        for j in 0..height {
            let b = bus.ram.read_byte(self.i + (j as u16));
            set_vf |= bus.display.draw_byte(b, x, y + j);
        }

        if set_vf {
            self.write_reg_v(0xF, 1);
        } else {
            self.write_reg_v(0xF, 0);
        }
    }
}

// Opcode Methods
impl Cpu {
    pub fn run_opcode(&mut self, bus: &mut Bus) {
        
        // Get Opcode
        let hi = bus.ram.read_byte(self.pc) as u16;
        let lo = bus.ram.read_byte(self.pc + 1) as u16;
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

        println!("opcode: {:#x}: {:?}", opcode, nibbles);

        let pc_change: ProgramCounter = match nibbles {
            (0x0, 0x0, 0x0, 0x0) => {
                // SYS addr (Call)
                // Jump to a machine code routine at nnn.
                panic!("F");
            }

            (0x0, 0x0, 0xE, 0x0) => {
                // CLS (Display)
                // Clear the display.

                bus.display.clear();

                ProgramCounter::Next
            }

            (0x0, 0x0, 0xE, 0xE) => {
                // RET (Flow)
                // Return from a subroutine.

                let addr = self.ret_stack.pop().unwrap();
                self.pc = addr;

                ProgramCounter::Next
            }

            (0x1, _, _, _) => {
                // JP addr (Flow)
                // Jump to location nnn.

                ProgramCounter::Jump(nnn)
            }

            (0x2, _, _, _) => {
                // CALL addr (Flow)
                // Call subroutine at nnn.
                self.ret_stack.push(self.pc + 2);

                ProgramCounter::Jump(nnn)
            }

            (0x3, _, _, _) => {
                // SE Vx, byte (Cond)
                // Skip next instruction if Vx = kk.

                let vx = self.read_reg_v(x);

                if vx == nn {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x4, _, _, _) => {
                // SNE Vx, byte (Cond)
                // Skip next instruction if Vx != kk.

                let vx = self.read_reg_v(x);
                if vx != nn {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x5, _, _, 0x0) => {
                // SE Vx, Vy (Cond)
                // Skip next instruction if Vx = Vy.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);

                if vx == vy {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x6, _, _, _) => {
                // LD Vx, byte (Const)
                // Set Vx = kk.

                self.write_reg_v(x, nn);

                ProgramCounter::Next
            }

            (0x7, _, _, _) => {
                // ADD Vx, byte (Const)
                // Set Vx = Vx + kk.

                let vx = self.read_reg_v(x);
                self.write_reg_v(x, vx.wrapping_add(nn));

                ProgramCounter::Next
            }

            (0x8, _, _, 0x0) => {
                // LD Vx, Vy (Assign)
                // Set Vx = Vy.

                let vy = self.read_reg_v(y);
                self.write_reg_v(x, vy);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x1) => {
                // OR Vx, Vy (BitOp)
                // Set Vx = Vx OR Vy.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                self.write_reg_v(x, vx | vy);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x2) => {
                // AND Vx, Vy (BitOp)
                // Set Vx = Vx AND Vy.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                self.write_reg_v(x, vx | vy);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy (BitOp)
                // Set Vx = Vx XOR Vy.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                self.write_reg_v(x, vx ^ vy);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy (Math)
                // Set Vx = Vx + Vy, set VF = carry.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                let sum: u16 = vx as u16 + vy as u16;

                // Write only the first 8 bits
                self.write_reg_v(x, sum as u8);
                // Set VF = Carry
                if sum > 0xFF {
                    self.write_reg_v(0xF, 1);
                }
                else {
                    self.write_reg_v(0xF, 1);
                }

                ProgramCounter::Next
            }

            (0x8, _, _, 0x5) => {
                // SUB Vx, Vy (Math)
                // Set Vx = Vx - Vy, set VF = NOT borrow.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                let diff: i8 = vx as i8 - vy as i8;

                // Write only the first 8 bits
                self.write_reg_v(x, diff as u8);
                // Set VF = NOT borrow
                if diff < 0 {
                    self.write_reg_v(0xF, 1);
                } else {
                    self.write_reg_v(0xF, 0);
                }

                ProgramCounter::Next
            }

            (0x8, _, _, 0x6) => {
                // SHR Vx {, Vy} (BitOp)
                // Set Vx = Vx SHR 1.

                let vx = self.read_reg_v(x);

                // Set VF = LSB(VX)
                self.write_reg_v(0xF, vx & 0x1);

                // SET VX = (VX >> 1)
                self.write_reg_v(x, vx >> 1);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy (Math)
                // Set Vx = Vy - Vx, set VF = NOT borrow.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                let diff: i8 = vy as i8 - vx as i8;

                // Write only the first 8 bits
                self.write_reg_v(x, diff as u8);
                if diff < 0 {
                    self.write_reg_v(0xF, 1);
                } else {
                    self.write_reg_v(0xF, 0);
                }

                ProgramCounter::Next
            }

            (0x8, _, _, 0xE) => {
                // SHL Vx {, Vy} (BitOp)
                // Set Vx = Vx SHL 1.

                let vx = self.read_reg_v(x);

                // Set VF = MSB(VX)
                self.write_reg_v(0xF, (vx & 0x80) >> 7);

                // SET VX = (VX << 1)
                self.write_reg_v(x, vx << 1);

                ProgramCounter::Next
            }

            (0x9, _, _, 0x0) => {
                // SNE Vx, Vy (Cond)
                // Skip next instruction if Vx != Vy.
                
                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);

                if vx != vy {
                    ProgramCounter::Skip
                }
                else {
                    ProgramCounter::Next
                }
            }

            (0xA, _, _, _) => {
                // LD I, addr (MEM)
                // Set I = nnn.

                self.i = nnn;

                ProgramCounter::Next
            }

            (0xB, _, _, _) => {
                // JP V0, addr (Flow)
                // Jump to location nnn + V0.

                self.pc = self.read_reg_v(0) as u16 + nnn;

                ProgramCounter::Next
            }

            (0xC, _, _, _) => {
                // RND Vx, byte (Rand)
                // Set Vx = random byte AND kk.
                ProgramCounter::Next
            }

            (0xD, _, _, _) => {
                // DRW Vx, Vy, nibble (Disp)
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                let vx = self.read_reg_v(x);
                let vy = self.read_reg_v(y);
                self.draw_sprite(bus, vx, vy, n);

                ProgramCounter::Next
            }

            (0xE, _, 0x9, 0xE) => {
                // SKP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is pressed.

                let key = self.read_reg_v(x);
                if bus.input.key_pressed(key) {
                    ProgramCounter::Skip
                }
                else {
                    ProgramCounter::Next
                }
            }

            (0xE, _, 0xA, 0x1) => {
                // SKNP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is not pressed.

                let key = self.read_reg_v(x);
                if bus.input.key_pressed(key) {
                    ProgramCounter::Next
                }
                else {
                    ProgramCounter::Skip
                }

            }

            (0xF, _, 0x0, 0x7) => {
                // LD Vx, DT (Timer)
                // Set Vx = delay timer value.

                self.write_reg_v(x, bus.get_delay_timer());

                ProgramCounter::Next
            }

            (0xF, _, 0x0, 0xA) => {
                // LD Vx, K (KeyOp)
                // Wait for a key press, store the value of the key in Vx.
                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0x5) => {
                // LD DT, Vx (Timer)
                // Set delay timer = Vx.

                let vx = self.read_reg_v(x);
                bus.set_delay_timer(vx);

                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0x8) => {
                // LD ST, Vx (Sound)
                // Set sound timer = Vx.
                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0xE) => {
                // ADD I, Vx (MEM)
                // Set I = I + Vx.

                let vx = self.read_reg_v(x);
                self.i += vx as u16;
                ProgramCounter::Next
            }

            (0xF, _, 0x2, 0x9) => {
                // LD F, Vx (MEM)
                // Set I = location of sprite for digit Vx.
                ProgramCounter::Next
            }

            (0xF, _, 0x3, 0x3) => {
                // LD B, Vx (BCD)
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                ProgramCounter::Next
            }

            (0xF, _, 0x5, 0x5) => {
                // LD [I], Vx (MEM)
                // Store registers V0 through Vx in memory starting at location I.
                ProgramCounter::Next
            }

            (0xF, _, 0x6, 0x5) => {
                // LD Vx, [I] (MEM)
                // Read registers V0 through Vx from memory starting at location I.

                for index in 0..x {
                    let value = bus.ram.read_byte(self.i + index as u16);
                    self.write_reg_v(index, value);
                }

                ProgramCounter::Next
            }
            _ => {
                panic!("Unrecognized Instruction")
            }
        };

        match pc_change {
            ProgramCounter::Next => self.pc += OPCODE_SIZE,
            ProgramCounter::Skip => self.pc += 2 * OPCODE_SIZE,
            ProgramCounter::Jump(addr) => self.pc = addr,
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
