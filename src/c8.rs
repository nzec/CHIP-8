pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const PROGRAM_START: usize = 0x200;
// The encoding for each pixel is 0RGB: The upper 8-bits are ignored, the next
// 8-bits are for the red channel, the next 8-bits afterwards for the green
// channel, and the lower 8-bits for the blue channel.
// Source: https://docs.rs/minifb/0.16.0/minifb/struct.Window.html#method.update_with_buffer
const PX_OFF: u32 = 0xff000000;
const PX_ON: u32 = 0xffffffff;

// Source: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
pub struct C8 {
    ram: [u8; RAM_SIZE],                // RAM
    pub v: [u8; REGISTER_COUNT],        // Vx Registers, VF = Special Flag
    pub display: [u32; WIDTH * HEIGHT], // Display Buffer
    i: u16,                             // Used to store Memory Addresses
    pub dt: u8,                         // Delay Timer
    pub st: u8,                         // Sound Timer
    stack: Vec<u16>,                    // Return Stack
    pub pc: u16,                        // Program Counter
}

// RAM Methods
impl C8 {
    pub fn load_ram(&mut self, rom: &Vec<u8>) {
        // Chip-8 draws graphics on screen through the use of sprites. A sprite
        // is a group of bytes which are a binary representation of the desired
        // picture. Chip-8 sprites may be up to 15 bytes, for a possible sprite
        // size of 8x15.
        // Programs may also refer to a group of sprites representing the
        // hexadecimal digits 0 through F. These sprites are 5 bytes long, or
        // 8x5 pixels. The data should be stored in the interpreter area of
        // Chip-8 memory (0x000 to 0x1FF).
        // Source: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];
        for la in 0..80 {
            let lc = sprites[la / 5][la % 5];
            println!("[Byte Load] | {:#x} : {:#x}", la, lc);
            self.ram[la] = lc;
        }

        for (j, lc) in rom.into_iter().enumerate() {
            let la = PROGRAM_START + j;
            if la >= RAM_SIZE {
                panic!("Out of memory: Program too large");
            }

            println!("[Byte Load] | {:#x} : {:#x}", la, lc);
            self.ram[la] = *lc;
        }
    }
}

// CPU Methods
impl C8 {
    pub fn run(&mut self, key_press: &[bool; 16]) -> usize {
        let mut wait_for_key: usize = 0;

        // All instructions are 2 bytes long and are stored
        // most-significant-byte first. In memory, the first byte of each
        // instruction should be located at an even addresses. If a program
        // includes sprite data, it should be padded so any instructions
        // following it will be properly situated in RAM.
        // Source: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
        let b1 = self.ram[self.pc as usize] as u16;
        let b2 = self.ram[self.pc as usize + 1] as u16;
        let inst = (b1 << 8) | b2;

        // In these listings, the following variables are used:
        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        // Source: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
        let nnn = (inst & 0x0FFF) as u16;
        let n = (inst & 0x000F) as u8;
        let x = ((inst & 0x0F00) >> 8) as usize;
        let y = ((inst & 0x00F0) >> 4) as usize;
        let kk = (inst & 0x00FF) as u8;

        let inst_tup = (
            ((inst & 0xF000) >> 12) as u8,
            ((inst & 0x0F00) >> 8) as u8,
            ((inst & 0x00F0) >> 4) as u8,
            (inst & 0x000F) as u8,
        );
        enum ProgramCounter {
            Next,
            Skip,
            Jump(u16),
        }

        let pc_change: ProgramCounter = match inst_tup {
            /*(0x0, _, _, _) => {
                // SYS addr (Call)
                // Jump to a machine code routine at nnn.
                // This instruction is only used on the old computers on which
                // Chip-8 was originally implemented. It is ignored by modern
                // interpreters.

                // As previously stated, CHIP-8 was originally implemented on
                // the RCA COSMAC VIP, and it was deemed desirable to include an
                // option to call machine language subroutines from a CHIP-8
                // program. The following instruction informs the CHIP-8
                // interpreter to execute a machine language program at a given
                // address, but it should be noted that this instruction is
                // highly considered deprecated, as it often remains
                // unimplemented on modern interpreters.
                // Source: http://mattmik.com/files/chip8/mastering/chip8.html

                ProgramCounter::Next
            }*/
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS (Display)
                // Clear the display.

                let sz = WIDTH * HEIGHT;
                for j in 0..sz {
                    self.display[j] = PX_OFF;
                }

                ProgramCounter::Next
            }

            (0x0, 0x0, 0xE, 0xE) => {
                // RET (Flow)
                // Return from a subroutine.

                // The interpreter sets the program counter to the address at
                // the top of the stack, then subtracts 1 from the stack
                // pointer.

                // This termination statement informs the interpreter that the
                // end of the currently executing subroutine has been reached,
                // and program execution should proceed at the point from which
                // the last subroutine call occurred.
                // Source: http://mattmik.com/files/chip8/mastering/chip8.html
                let addr = self
                    .stack
                    .pop()
                    .expect("Empty Stack: Cannot return from Subroutine");
                self.pc = addr;

                ProgramCounter::Next
            }

            (0x1, _, _, _) => {
                // JP addr (Flow)
                // Jump to location nnn.

                // The interpreter sets the program counter to nnn.

                ProgramCounter::Jump(nnn)
            }

            (0x2, _, _, _) => {
                // CALL addr (Flow)
                // Call subroutine at nnn.

                // The interpreter increments the stack pointer, then puts the
                // current PC on the top of the stack. The PC is then set to
                // nnn.

                // CHIP-8 program execution will then continue from this address
                // until a termination instruction is found.
                // Source: http://mattmik.com/files/chip8/mastering/chip8.html
                self.stack.push(self.pc);

                ProgramCounter::Jump(nnn)
            }

            (0x3, _, _, _) => {
                // SE Vx, byte (Cond)
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are
                // equal, increments the program counter by 2.

                if self.v[x] == kk {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x4, _, _, _) => {
                // SNE Vx, byte (Cond)
                // Skip next instruction if Vx != kk.

                // The interpreter compares register Vx to kk, and if they are
                // not equal, increments the program counter by 2.

                if self.v[x] != kk {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x5, _, _, 0x0) => {
                // SE Vx, Vy (Cond)
                // Skip next instruction if Vx = Vy.

                // The interpreter compares register Vx to register Vy, and if
                // they are equal, increments the program counter by 2.

                if self.v[x] == self.v[y] {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0x6, _, _, _) => {
                // LD Vx, byte (Const)
                // Set Vx = kk.

                // The interpreter puts the value kk into register Vx.

                self.v[x] = kk;

                ProgramCounter::Next
            }

            (0x7, _, _, _) => {
                // ADD Vx, byte (Const)
                // Set Vx = Vx + kk.

                // Adds the value kk to the value of register Vx, then stores
                // the result in Vx.

                // Be aware that once the supplied number is added, if the value
                // of the register exceeds decimal 255 (the highest possible
                // value that can be stored by an eight bit register), the
                // register will wraparound to a corresponding value that can be
                // stored by an eight bit register. In other words, the register
                // will always be reduced modulo decimal 256.
                // Source: http://mattmik.com/files/chip8/mastering/chip8.html
                // Wrapping (modular) addition. Computes self + rhs, wrapping
                // around at the boundary of the type.
                // Source: https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add
                self.v[x] = self.v[x].wrapping_add(kk);

                ProgramCounter::Next
            }

            (0x8, _, _, 0x0) => {
                // LD Vx, Vy (Assign)
                // Set Vx = Vy.

                // Stores the value of register Vy in register Vx.

                self.v[x] = self.v[y];

                ProgramCounter::Next
            }

            (0x8, _, _, 0x1) => {
                // OR Vx, Vy (BitOp)
                // Set Vx = Vx OR Vy.

                // Performs a bitwise OR on the values of Vx and Vy, then stores
                // the result in Vx. A bitwise OR compares the corrseponding
                // bits from two values, and if either bit is 1, then the same
                // bit in the result is also 1. Otherwise, it is 0.

                self.v[x] = self.v[x] | self.v[y];

                ProgramCounter::Next
            }

            (0x8, _, _, 0x2) => {
                // AND Vx, Vy (BitOp)
                // Set Vx = Vx AND Vy.

                // Performs a bitwise AND on the values of Vx and Vy, then
                // stores the result in Vx. A bitwise AND compares the
                // corrseponding bits from two values, and if both bits are 1,
                // then the same bit in the result is also 1. Otherwise, it is
                // 0.

                self.v[x] = self.v[x] & self.v[y];

                ProgramCounter::Next
            }

            (0x8, _, _, 0x3) => {
                // XOR Vx, Vy (BitOp)
                // Set Vx = Vx XOR Vy.

                // Performs a bitwise exclusive OR on the values of Vx and Vy,
                // then stores the result in Vx. An exclusive OR compares the
                // corrseponding bits from two values, and if the bits are not
                // both the same, then the corresponding bit in the result is
                // set to 1. Otherwise, it is 0.

                self.v[x] = self.v[x] ^ self.v[y];

                ProgramCounter::Next
            }

            (0x8, _, _, 0x4) => {
                // ADD Vx, Vy (Math)
                // Set Vx = Vx + Vy, set VF = carry.

                // The values of Vx and Vy are added together. If the result is
                // greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise
                // 0. Only the lowest 8 bits of the result are kept, and stored
                // in Vx.

                // Calculates self + rhs
                // Returns a tuple of the addition along with a boolean
                // indicating whether an arithmetic overflow would occur. If an
                // overflow would have occurred then the wrapped value is
                // returned.
                // Source: https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_add
                let (res, over) = self.v[x].overflowing_add(self.v[y]);

                self.v[x] = res;
                self.v[0xF] = over as u8;

                ProgramCounter::Next
            }

            (0x8, _, _, 0x5) => {
                // SUB Vx, Vy (Math)
                // Set Vx = Vx - Vy, set VF = NOT borrow.

                // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is
                // subtracted from Vx, and the results stored in Vx.

                // Calculates self - rhs
                // Returns a tuple of the subtraction along with a boolean
                // indicating whether an arithmetic overflow would occur. If an
                // overflow would have occurred then the wrapped value is
                // returned.
                // Source: https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_sub
                let (res, over) = self.v[x].overflowing_sub(self.v[y]);

                self.v[x] = res;
                self.v[0xF] = !over as u8;

                ProgramCounter::Next
            }

            (0x8, _, _, 0x6) => {
                // SHR Vx {, Vy} (BitOp)
                // Set Vx = Vx SHR 1.

                // If the least-significant bit of Vx is 1, then VF is set to 1,
                // otherwise 0. Then Vx is divided by 2.

                // Panic-free bitwise shift-right; yields self >> mask(rhs),
                // where mask removes any high-order bits of rhs that would
                // cause the shift to exceed the bitwidth of the type.
                // Note that this is not the same as a rotate-right; the RHS of
                // a wrapping shift-right is restricted to the range of the
                // type, rather than the bits shifted out of the LHS being
                // returned to the other end. The primitive integer types all
                // implement a rotate_right function, which may be what you want
                // instead.
                // Source: https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_shr
                let res = self.v[x].wrapping_shr(1);

                self.v[0xF] = self.v[x] & 0b1;
                self.v[x] = res;

                ProgramCounter::Next
            }

            (0x8, _, _, 0x7) => {
                // SUBN Vx, Vy (Math)
                // Set Vx = Vy - Vx, set VF = NOT borrow.

                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is
                // subtracted from Vy, and the results stored in Vx.

                // Calculates self - rhs
                // Returns a tuple of the subtraction along with a boolean
                // indicating whether an arithmetic overflow would occur. If an
                // overflow would have occurred then the wrapped value is
                // returned.
                // Source: https://doc.rust-lang.org/std/primitive.u8.html#method.overflowing_sub
                let (res, over) = self.v[y].overflowing_sub(self.v[x]);

                self.v[x] = res;
                self.v[0xF] = !over as u8;

                ProgramCounter::Next
            }

            (0x8, _, _, 0xE) => {
                // SHL Vx {, Vy} (BitOp)
                // Set Vx = Vx SHL 1.

                // If the most-significant bit of Vx is 1, then VF is set to 1,
                // otherwise to 0. Then Vx is multiplied by 2.

                // Panic-free bitwise shift-left; yields self << mask(rhs),
                // where mask removes any high-order bits of rhs that would
                // cause the shift to exceed the bitwidth of the type.
                // Note that this is not the same as a rotate-left; the RHS of a
                // wrapping shift-left is restricted to the range of the type,
                // rather than the bits shifted out of the LHS being returned to
                // the other end. The primitive integer types all implement a
                // rotate_left function, which may be what you want instead.
                // Source: https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_shl
                let res = self.v[x].wrapping_shl(1);

                // 128 = 0b1000_0000 = 0x80
                self.v[0xF] = (self.v[x] & 0x80) >> 7;
                self.v[x] = res;

                ProgramCounter::Next
            }

            (0x9, _, _, 0x0) => {
                // SNE Vx, Vy (Cond)
                // Skip next instruction if Vx != Vy.

                // The values of Vx and Vy are compared, and if they are not
                // equal, the program counter is increased by 2.

                if self.v[x] != self.v[y] {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0xA, _, _, _) => {
                // LD I, addr (MEM)
                // Set I = nnn.

                // The value of register I is set to nnn.

                self.i = nnn;

                ProgramCounter::Next
            }

            (0xB, _, _, _) => {
                // JP V0, addr (Flow)
                // Jump to location nnn + V0.

                // The program counter is set to nnn plus the value of V0.

                ProgramCounter::Jump(self.v[0] as u16 + nnn)
            }

            (0xC, _, _, _) => {
                // RND Vx, byte (Rand)
                // Set Vx = random byte AND kk.

                // The interpreter generates a random number from 0 to 255,
                // which is then ANDed with the value kk. The results are stored
                // in Vx. See instruction 8xy2 for more information on AND.

                let rnd = rand::random::<u8>();
                self.v[x] = rnd & kk;

                ProgramCounter::Next
            }

            (0xD, _, _, _) => {
                // DRW Vx, Vy, nibble (Disp)
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                // The interpreter reads n bytes from memory, starting at the
                // address stored in I. These bytes are then displayed as sprites
                // on screen at coordinates (Vx, Vy). Sprites are XORed onto the
                // existing screen. If this causes any pixels to be erased, VF is
                // set to 1, otherwise it is set to 0. If the sprite is
                // positioned so part of it is outside the coordinates of the
                // display, it wraps around to the opposite side of the screen.
                // See instruction 8xy3 for more information on XOR, and section
                // 2.4, Display, for more information on the Chip-8 screen and
                // sprites.

                let inx = self.v[x];
                let iny = self.v[y];
                let mut collision = 0;

                let mut printer = Vec::<u8>::new();
                for j in 0..n as usize {
                    printer.push(self.ram[self.i as usize + j]);
                }

                for (k, b) in printer.iter().enumerate() {
                    for j in 0..8 {
                        let cux = inx.wrapping_add(j) % WIDTH as u8;
                        let cuy = iny.wrapping_add(k as u8) % HEIGHT as u8;

                        let cord = (cuy as usize * WIDTH) + cux as usize;
                        let is_old_set = self.display[cord] == PX_ON;

                        self.display[cord] = if (b >> (7 - j)) & 0b1 == 1 {
                            if is_old_set {
                                collision |= 1;
                                PX_OFF
                            } else {
                                PX_ON
                            }
                        } else {
                            self.display[cord]
                        }
                    }
                }

                self.v[0xF] = collision;

                ProgramCounter::Next
            }

            (0xE, _, 0x9, 0xE) => {
                // SKP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is pressed.

                // Checks the keyboard, and if the key corresponding to the
                // value of Vx is currently in the down position, PC is
                // increased by 2.

                if key_press[self.v[x] as usize] {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0xE, _, 0xA, 0x1) => {
                // SKNP Vx (KeyOp)
                // Skip next instruction if key with the value of Vx is not pressed.

                // Checks the keyboard, and if the key corresponding to the
                // value of Vx is currently in the up position, PC is increased
                // by 2.

                if !key_press[self.v[x] as usize] {
                    ProgramCounter::Skip
                } else {
                    ProgramCounter::Next
                }
            }

            (0xF, _, 0x0, 0x7) => {
                // LD Vx, DT (Timer)
                // Set Vx = delay timer value.

                // The value of DT is placed into Vx.

                self.v[x] = self.dt;

                ProgramCounter::Next
            }

            (0xF, _, 0x0, 0xA) => {
                // LD Vx, K (KeyOp)
                // Wait for a key press, store the value of the key in Vx.

                // All execution stops until a key is pressed, then the value of
                // that key is stored in Vx.

                wait_for_key = x;

                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0x5) => {
                // LD DT, Vx (Timer)
                // Set delay timer = Vx.

                // All execution stops until a key is pressed, then the value of
                // that key is stored in Vx.

                self.dt = self.v[x];

                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0x8) => {
                // LD ST, Vx (Sound)
                // Set sound timer = Vx.

                // ST is set equal to the value of Vx.

                self.st = self.v[x];

                ProgramCounter::Next
            }

            (0xF, _, 0x1, 0xE) => {
                // ADD I, Vx (MEM)
                // Set I = I + Vx.

                // The values of I and Vx are added, and the results are stored
                // in I.

                self.i += self.v[x] as u16;

                ProgramCounter::Next
            }

            (0xF, _, 0x2, 0x9) => {
                // LD F, Vx (MEM)
                // Set I = location of sprite for digit Vx.

                // The value of I is set to the location for the hexadecimal
                // sprite corresponding to the value of Vx. See section 2.4,
                // Display, for more information on the Chip-8 hexadecimal font.

                // Mutliplying by 5 because each character occupies 5
                // consecutive spots starting from 0.
                self.i = self.v[x] as u16 * 5;
                ProgramCounter::Next
            }

            (0xF, _, 0x3, 0x3) => {
                // LD B, Vx (BCD)
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.

                // The interpreter takes the decimal value of Vx, and places the
                // hundreds digit in memory at location in I, the tens digit at
                // location I+1, and the ones digit at location I+2.

                self.ram[self.i as usize] = self.v[x] / 100;
                self.ram[self.i as usize + 1] = (self.v[x] % 100) / 10;
                self.ram[self.i as usize + 2] = self.v[x] % 10;

                ProgramCounter::Next
            }

            (0xF, _, 0x5, 0x5) => {
                // LD [I], Vx (MEM)
                // Store registers V0 through Vx in memory starting at location I.

                // The interpreter copies the values of registers V0 through Vx
                // into memory, starting at the address in I.

                for j in 0..=x {
                    self.ram[self.i as usize + j] = self.v[j];
                }

                ProgramCounter::Next
            }

            (0xF, _, 0x6, 0x5) => {
                // LD Vx, [I] (MEM)
                // Read registers V0 through Vx from memory starting at location I.

                // The interpreter reads values from memory starting at location
                // I into registers V0 through Vx.

                for j in 0..=x {
                    self.v[j] = self.ram[self.i as usize + j];
                }

                ProgramCounter::Next
            }
            _ => {
                println!("Warning: unrecognized instruction: {:04x}", inst);
                ProgramCounter::Next
            }
        };

        match pc_change {
            ProgramCounter::Next => self.pc += 2,
            ProgramCounter::Skip => self.pc += 4,
            ProgramCounter::Jump(addr) => self.pc = addr,
        }

        println!(
            "[Executing] | pc: {:#03x} | inst: {:#04x} | i: {:#04x} | v: {:02x?}",
            self.pc, inst, self.i, self.v
        );

        wait_for_key
    }
}

// New
impl C8 {
    pub fn new() -> C8 {
        C8 {
            ram: [0; RAM_SIZE],
            v: [0; REGISTER_COUNT],
            display: [PX_OFF; WIDTH * HEIGHT],
            i: 0,
            dt: 0,
            st: 0,
            stack: Vec::new(),
            pc: PROGRAM_START as u16,
        }
    }
}
