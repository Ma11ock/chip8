/// Chip 8 emulator.
/// (C) Ryan Jeffrey <ryan@ryanmj.xyz>, 2022
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
#[cfg(test)]
mod main_test;
extern crate sdl2;
extern crate rand;

use std::env;
use std::fs;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::rngs::ThreadRng;

#[allow(arithmetic_overflow)]

/// Windows width in pixels.
const WIN_WIDTH: u32 = 800;
/// Windows height in pixels.
const WIN_HEIGHT: u32 = 400;

/// The number of rows on the screen (chip 8 height).
const NUM_ROWS: usize = 32;
/// The number of columns on the screen (chip 8 width).
const NUM_COLS: usize = 64;

/// From http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#dispcoords
/// The chip 8 font sprites.
const FONTSET: [u8; 0x10 * 5] = [
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

/// Interpreter state.
struct InterpreterData {
    /// V registers. 16 of them, general purpose, 8 bits.
    v: [u8; 16],
    /// I register, 16 bits.
    i: u16,
    /// Program counter, 16 bits.
    pc: u16,
    /// Stack pointer, 16 bits.
    sp: u8,
    /// The call stack, 16 levels of 16 bits each.
    stack: [u16; 16],
    /// Delay timer @ 60Hz, 8 bits.
    delay_timer: u8,
    /// Sound timer @ 60Hz, 8 bits.
    sound_timer: u8,
    /// Memory.
    mem: [u8; 4096],
    /// The screen.
    screen: [[bool; NUM_ROWS]; NUM_COLS],
    /// Redraw the screen flag.
    draw: bool,
    /// Rng.
    rng: ThreadRng,
}

impl InterpreterData {
    /// Create a new Interpreter state struct, 0 initialize.
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            mem: [0; 4096],
            screen: [[false; NUM_ROWS]; NUM_COLS],
            draw: false,
            rng: rand::thread_rng(),
        }
    }

    /// Pop the stack and return the memory address on top of it.
    fn pop_stack(&mut self) -> u16 {
        let r = self.stack[self.sp as usize];
        self.sp -= 1;
        r
    }

    /// Push current memory address to the stack.
    fn push_stack(&mut self) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
    }

    /// Get the value of register reg.
    /// # Arguments
    /// * `reg` The register to return. Valid from 0-0xf.
    fn get_register(&self, reg: u8) -> u8 {
        self.v[reg as usize]
    }

    /// Set the value of register reg.
    /// # Arguments
    /// * `reg` The register to set. Valid from 0-0xf.
    /// * `value` The value to place in the register.
    fn set_register(&mut self, reg: u8, value: u8) {
        self.v[reg as usize] = value;
    }

    /// Add `amount` to the current program counter, return result.
    /// # Arguments
    /// * `amount` The amount to add to the program counter.
    fn increment_pc(&self, amount: u16) -> u16 {
        self.pc + amount
    }
}

/// Chip 8 instruction and their arguments.
#[derive(PartialEq, Eq, Copy, Clone)]
enum Instruction {
    /// Sys, ignored in this emulator.
    Sys(u16),
    /// Clear the screen.
    Cls,
    /// Return from the current subroutine, pop the stack.
    Ret,
    /// Jump to memory address.
    Jp(u16),
    /// Call subroutine, push to the stack.
    Call(u16),
    /// Skip next instruction if register equals value.
    Se(u8, u8),
    /// Skip next instruction if register does not equal value.
    Sne(u8, u8),
    /// Skip next instruction if registers are equal.
    SeR(u8, u8),
    /// Take register and load bottom byte.
    Ld(u8, u8),
    /// Add register to bottom byte.
    Add(u8, u8),
    /// Load register into other register.
    LdR(u8, u8),
    /// Bitwise OR registers.
    Or(u8, u8),
    /// Bitwise And registers.
    And(u8, u8),
    /// Bitwise Xor registers.
    Xor(u8, u8),
    /// Add first register to second register, store in first register.
    AddR(u8, u8),
    /// Subtract register to bottom byte.
    Sub(u8, u8),
    /// Divide register value by 2.
    Shr(u8, u8),
    /// Subtract second register by first register.
    SubN(u8, u8),
    /// Multiply register by 2.
    Shl(u8, u8),
    /// Skip next instruction if two registers do not equal.
    SneR(u8, u8),
    /// Load value into I register.
    LdI(u16),
    /// Jump to value + I.
    JpI(u16),
    /// Place random number AND bottom byte into register.
    Rnd(u8, u8),
    /// Draw n byte sprite at position gained from first two registers.
    Drw(u8, u8, u8),
    /// Skip next instruction if key is pressed.
    Skp(u8),
    /// Skip next instruction if key is not pressed.
    SkpN(u8),
    /// Load the value of the delay timer into register.
    LdD(u8),
    /// Load keypress, halt until key is pressed.
    LdW(u8),
    /// Set delay time value.
    LdSD(u8),
    /// Set sound time value.
    LdS(u8),
    /// Add value of register with I register, store in first register.
    AddI(u8),
    /// Load sprite location from V[x].
    LdSp(u8),
    /// Store BCD repr of V[x] in I, I + 1, I + 2.
    LdBCD(u8),
    /// Store registers V[0] to V[x] in memory starting at I.
    LdIR(u8),
    /// Store memory at I into V[0] to V[x].
    LdIRM(u8),
}

/// Instruction interpretation error.
enum InstructionError {
    /// Instruction interpretation error.
    InvalidInstruction
}

/// Return the bottom three nibbles from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_last_3_nibbles(n: u16) -> u16 {
    n & 0x0FFFu16
}

/// Return the bottom byte from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_last_2_nibbles(n: u16) -> u8 {
    (n & 0x00FFu16) as u8
}


/// Return the top second nibble from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_third_nibble(n: u16) -> u8 {
    ((n & 0x0F00u16) >> 8) as u8
}

/// Return the top third nibble from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_second_nibble(n: u16) -> u8 {
    ((n & 0x00F0u16) >> 4) as u8
}

/// Return the bottom nibble from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_first_nibble(n: u16) -> u8 {
    (n & 0x000fu16) as u8
}

/// Return the top nibble from opcode.
/// # Arguments
/// * `n` 16 bit opcode.
fn get_fourth_nibble(n: u16) -> u8 {
    ((n & 0xf000u16) >> 12) as u8
}

/// Return conversion from memory address to Instruction position.
/// # Arguments
/// * `n` Memory address.
fn jp_to_instruction_pos(d: u16) -> u16 {
    (d - 0x200) / 2
}

/// Emulate chip8 instruction at emu_state's program counter. 
/// # Arguments
/// * `program` The chip8 program as Instructions.
/// * `emu_state` The emulator state to change.
/// * `cur_pressed_keys` Keypad state.
fn emulate(program: &Vec<Instruction>, emu_state: &mut InterpreterData,
           cur_pressed_keys: &[bool; 0x10]) {
    type I = Instruction;

    let instruction = program[emu_state.pc as usize];

    // Check first nibble, store result of match in the program counter.
    emu_state.pc = match instruction {
        I::Sys(..) => {
            // Ignored on modern interpreters.
            emu_state.increment_pc(1)
        },
        I::Cls => {
            // Clear the display.
            emu_state.screen = [[false; NUM_ROWS]; NUM_COLS];
            emu_state.increment_pc(1)
        },
        I::Ret => {
            // Set PC to to stack[sp], decrement sp.
            emu_state.pop_stack() + 1
        },
        I::Jp(nnn) => {
            nnn
        },
        // Function call at bottom three nibbles.
        I::Call(nnn) => {
            emu_state.push_stack();
            nnn
        },
        // Skip next instruction if the bottom byte is equal to the value
        // in V[first nibble].
        I::Se(x, kk) => {
            if emu_state.get_register(x) == kk {
                emu_state.increment_pc(2)
            } else {
                emu_state.increment_pc(1)
            }
        },
        // Skip next instruction if V[third nibble] == bottom byte.
        I::Sne(x, kk) => {
            if emu_state.get_register(x) != kk {
                emu_state.increment_pc(2)
            } else {
                emu_state.increment_pc(1)
            }
        },
        // If V[third nibble] == V[second nibble] then skip next instruction.
        I::SeR(x, y) => {
            if emu_state.get_register(x) == emu_state.get_register(y) {
                    emu_state.increment_pc(2)
                } else {
                    emu_state.increment_pc(1)
                }
        },
        // Put the bottom byte into register V[third nibble].
        I::Ld(x, kk) => {
            emu_state.set_register(x, kk);
            emu_state.increment_pc(1)
        },
        // Adds the bottom byte to the value of V[third nibble], then
        // stores it there.
        I::Add(x, kk) => {
            emu_state.set_register(x, emu_state.get_register(x) + kk);
            emu_state.increment_pc(1)
        },
        I::LdR(x, y) => {
            emu_state.set_register(x, emu_state.get_register(y));
            emu_state.increment_pc(1)
        },
        I::Or(x, y) => {
            emu_state.set_register(x,
                                   emu_state.get_register(x) |
                                   emu_state.get_register(y));
            emu_state.increment_pc(1)
        },
        I::And(x, y) => {
            emu_state.set_register(x,
                                   emu_state.get_register(x) &
                                   emu_state.get_register(y));
            emu_state.increment_pc(1)
        },
        I::Xor(x, y) => {
            emu_state.set_register(x,
                                   emu_state.get_register(x) ^
                                   emu_state.get_register(y));
            emu_state.increment_pc(1)
        },
        I::AddR(x, y) => {
            emu_state.set_register(x,
                                   emu_state.get_register(x) +
                                   emu_state.get_register(y));
            if emu_state.get_register(x) < emu_state.get_register(y) {
                emu_state.set_register(0xf, 1);
            }
            emu_state.increment_pc(1)
        },
        I::Sub(x, y) => {
            if emu_state.get_register(x) > emu_state.get_register(y) {
                emu_state.set_register(0xf, 1);
            }
            emu_state.set_register(x,
                                   emu_state.get_register(x) -
                                   emu_state.get_register(y));
            emu_state.increment_pc(1)
        },
        I::Shr(x, y) => {
            if emu_state.get_register(y) & 1 == 1 {
                emu_state.set_register(0xf, 1);
            }
            emu_state.set_register(x,  emu_state.get_register(y) >> 1);
            emu_state.increment_pc(1)
        },
        I::SubN(x, y) => {
            if emu_state.get_register(x) < emu_state.get_register(y) {
                emu_state.set_register(0xf, 1);
            }
            emu_state.set_register(x,
                                   emu_state.get_register(y) -
                                   emu_state.get_register(x));
            emu_state.increment_pc(1)
        },
        I::Shl(x, y) => {
            if emu_state.get_register(y) & 0x80 != 0 {
                emu_state.set_register(0xf, 1);
            }
            emu_state.set_register(x,
                                   emu_state.get_register(y) << 1);
            emu_state.increment_pc(1)
        },
        I::SneR(x, y) => {
            if emu_state.get_register(x) != emu_state.get_register(y) {
                emu_state.increment_pc(2)
            } else {
                emu_state.increment_pc(1)
            }
        },
        I::LdI(nnn) => {
            emu_state.i = nnn;
            emu_state.increment_pc(1)
        },
        I::JpI(nnn) => {
            nnn + emu_state.get_register(0) as u16
        },
        I::Rnd(x, kk) => {
            let rn = emu_state.rng.gen::<u8>();
            emu_state.set_register(x, rn & kk);
            emu_state.increment_pc(1)
        },
        // Display n-byte sprite starting at memory location I at (Vx, Vy),
        // set VF = collision.
        I::Drw(x, y, n) => {
            emu_state.set_register(0xf, 0);
            for i in 0..(n as usize) {
                let sb = emu_state.mem[emu_state.i as usize + i];
                for j in 0..8 {
                    let xj = (emu_state.get_register(x) as usize + j) % NUM_COLS;
                    let yi = (emu_state.get_register(y) as usize + i) % NUM_ROWS;
                    if sb & (0x80 >> j) != 0 {
                        if emu_state.screen[xj][yi] {
                            emu_state.set_register(0xf, 1);
                            emu_state.screen[xj][yi] = false;
                        } else {
                            emu_state.screen[xj][yi] = true;
                        }
                    }
                }
            }
            emu_state.draw = true;
            emu_state.increment_pc(1)
        },
        I::Skp(x) => {
            if cur_pressed_keys[emu_state.get_register(x) as usize] {
                emu_state.increment_pc(2)
            } else {
                emu_state.increment_pc(1)
            }
        },
        I::SkpN(x) => {
            if !cur_pressed_keys[emu_state.get_register(x) as usize] {
                emu_state.increment_pc(2)
            } else {
                emu_state.increment_pc(1)
            }
        },
        I::LdD(x) => {
            emu_state.set_register(x, emu_state.delay_timer);
            emu_state.increment_pc(1)
        },
        I::LdW(_) => {
            // TODO
            emu_state.increment_pc(1)
        },
        I::LdSD(x) => {
            emu_state.delay_timer = emu_state.get_register(x);
            emu_state.increment_pc(1)
        },
        I::LdS(x) => {
            emu_state.sound_timer = emu_state.get_register(x);
            emu_state.increment_pc(1)
        },
        I::AddI(x) => {
            emu_state.i = emu_state.i + emu_state.get_register(x) as u16;
            emu_state.increment_pc(1)
        },
        I::LdSp(x) => {
            emu_state.i = 5 * emu_state.get_register(x) as u16;
            emu_state.increment_pc(1)
        },
        I::LdBCD(x) => {
            let n = emu_state.get_register(x);
            let i = emu_state.i as usize;
            emu_state.mem[i] = (n / 100) % 10;
            emu_state.mem[i + 1] = (n / 10) % 10;
            emu_state.mem[i + 2] = n % 10;
            emu_state.increment_pc(1)
        },
        I::LdIR(x) => {
            for i in 0..=(x as u16) {
                emu_state.mem[(emu_state.i + i) as usize] =
                    emu_state.get_register(i as u8);
            }
            emu_state.increment_pc(1)
        },
        I::LdIRM(x) => {
            for i in 0..=(x as u16) {
                emu_state.set_register(i as u8,
                                       emu_state.mem[(emu_state.i + i) as usize]);
            }
            emu_state.increment_pc(1)
        },
    };
}

#[cfg(test)]
/// Run an entire program, exists for unit tests.
/// # Arguments
/// * `program` The program to run.
/// * `emu_state` Emulator state to change.
fn emulate_program(program: &Vec<Instruction>, emu_state: &mut InterpreterData) {
    for _ in program {
        emulate(&program, emu_state, &[false; 0x10]);
    }
}

/// Convert raw chip 8 opcode into instruction. Returns InstructionError
/// if instruction is invalid.
/// # Arguments
/// `instruction` Chip 8 opcode.
fn program_to_enum(instruction: u16) -> Result<Instruction, InstructionError> {
    type I = Instruction;
    Ok(match get_fourth_nibble(instruction) {
        0 => {
            match get_last_2_nibbles(instruction) {
                0xE0 => I::Cls,
                0xEE => I::Ret,
                _ => I::Sys(get_last_3_nibbles(instruction)),
            }
        },
        // Set PC to bottom three nibbles.
        1 => {
            I::Jp(jp_to_instruction_pos(get_last_3_nibbles(instruction)))
        },
        // Function call at bottom three nibbles.
        2 => {
            I::Call(jp_to_instruction_pos(get_last_3_nibbles(instruction)))
        },
        // Skip next instruction if the bottom byte is equal to the value
        // in V[first nibble].
        3 => {
            I::Se(get_third_nibble(instruction), get_last_2_nibbles(instruction))
        },
        // Skip next instruction if V[third nibble] == bottom byte.
        4 => {
            I::Sne(get_third_nibble(instruction), get_last_2_nibbles(instruction))
        },
        // If V[third nibble] == V[second nibble] then skip next instruction.
        5 => {
            if get_first_nibble(instruction) == 0 {
                I::SeR(get_third_nibble(instruction), get_second_nibble(instruction))
            } else {
                return Err(InstructionError::InvalidInstruction);
            }
        },
        // Put the bottom byte into register V[third nibble].
        6 => {
            I::Ld(get_third_nibble(instruction), get_last_2_nibbles(instruction))
        },
        // Adds the bottom byte to the value of V[third nibble], then
        // stores it there.
        7 => {
            I::Add(get_third_nibble(instruction), get_last_2_nibbles(instruction))
        },
        8 => {
            match get_first_nibble(instruction) {
                // Bitwise OR V[third nibble] and V[second nibble], store
                // result in V[third nibble].
                0 => {
                    I::LdR(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                1 => {
                    I::Or(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                2 => {
                    I::And(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                3 => {
                    I::Xor(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                4 => {
                    I::AddR(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                5 => {
                    I::Sub(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                6 => {
                    I::Shr(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                7 => {
                    I::SubN(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                0xe => {
                    I::Shl(get_third_nibble(instruction), get_second_nibble(instruction))
                },
                _ => return Err(InstructionError::InvalidInstruction),
            }
        },
        9 => {
            if get_first_nibble(instruction) == 0 {
                I::SneR(get_third_nibble(instruction), get_second_nibble(instruction))
            } else {
                return Err(InstructionError::InvalidInstruction);
            }
        },
        0xa => {
            I::LdI(get_last_3_nibbles(instruction))
        },
        0xb => {
            I::JpI(jp_to_instruction_pos(get_last_3_nibbles(instruction)))
        },
        0xc => {
            I::Rnd(get_third_nibble(instruction), get_last_2_nibbles(instruction))
        },
        0xd => {
            I::Drw(get_third_nibble(instruction),
                get_second_nibble(instruction),
                get_first_nibble(instruction))
        },
        0xe => {
            match get_first_nibble(instruction) {
                0xe => I::Skp(get_third_nibble(instruction)),
                0x1 => I::SkpN(get_third_nibble(instruction)),
                _ => return Err(InstructionError::InvalidInstruction),
            }
        },
        0xf => {
            let third_nibble = get_third_nibble(instruction);
            match get_last_2_nibbles(instruction) {
                0x07 => {
                    I::LdD(third_nibble)
                },
                0x0a => {
                    I::LdW(third_nibble)
                },
                0x15 => {
                    I::LdSD(third_nibble)
                },
                0x18 => {
                    I::LdS(third_nibble)
                },
                0x1E => {
                    I::AddI(third_nibble)
                },
                0x29 => {
                    I::LdSp(third_nibble)
                },
                0x33 => {
                    I::LdBCD(third_nibble)
                },
                0x55 => {
                    I::LdIR(third_nibble)
                },
                0x65 => {
                    I::LdIRM(third_nibble)
                },
                _ => return Err(InstructionError::InvalidInstruction),
            }
        },
        // Load bottom byte into V[third nibble].
        _ => return Err(InstructionError::InvalidInstruction),
    })
}

/// Convert vector of chip 8 opcodes into vector of instructions. Returns
/// string on error.
/// # Arguments 
/// * `data` Raw chip 8 opcode vector.
fn convert_program(data: &Vec<u16>) -> Result<Vec<Instruction>, String> {
    // HACK this is a bad design. Not only does it mess with JP and CALL
    // instructions, it also has no way of differentiating sprite/constant
    // data with actual instructions. A design to avoid in the future.
    let mut result: Vec<Instruction> = Vec::with_capacity(data.len());
    for i in data.iter() {
        match program_to_enum(*i) {
            Ok(d) => result.push(d),
            // Ignore "invalid instructions", as they could just be sprite data.
            // Need to push an instruction to preserve order.
            _ => result.push(Instruction::Sys(0)),
        }
    }
    Ok(result)
}

/// Return the name of the file from command line arguments. If no file was
/// specified the default to game.bin.
fn get_bin_file() -> String {
    const DEFAULT_FILE: &str = "game.bin";
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match args.last() {
            Some(a) => a.to_string(),
            None => String::from(DEFAULT_FILE),
        }
    } else {
        String::from(DEFAULT_FILE)
    }
}

/// Convert byte stream into 16 bit opcode array.
/// Return vector of u16 on success, return string on error.
fn convert_bin_format(bytes: &[u8]) -> Result<Vec<u16>, String> {
    // Because chip8 instructions are 16 bits its length (in bytes) should be even.
    if bytes.len() % 2 == 1 {
        return Err(String::from("Invalid input: the file is not an even length."));
    }

    let mut result: Vec<u16> = Vec::with_capacity(bytes.len() / 2);

    for i in (0..bytes.len()).step_by(2) {
        if cfg!(target_endian = "big") {
            result.push(bytes[i] as u16 | (bytes[i + 1] as u16) << 8);
        } else {
            result.push((bytes[i] as u16) << 8 | bytes[i + 1] as u16);
        }
    }

    Ok(result)
}

/// Get the program as a vector of instructions and as a raw byte stream.
/// Return vectors on success, return string on error.
fn get_program() -> Result<(Vec<Instruction>, Vec<u8>), String>  {
    let game_file: String = get_bin_file();
    println!("Opening binary file {}.", game_file);

    match fs::read(game_file) {
        Ok(mut raw_p) => {
            // Ensure raw_p is even length.
            if raw_p.len() % 2 == 1 {
                raw_p.push(0);
            }
            Ok((convert_program(&convert_bin_format(&raw_p)?)?, raw_p))
        },
        Err(raw_e) => return Err(raw_e.to_string()),
    }
}

/// Draw the emulator state to the SDL screen. Return string on error.
/// # Arguments
/// * `emu_state` Raw emulator state to draw.
/// * `canvas` SDL canvas to draw to.
fn draw_screen(emu_state: &InterpreterData, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let cell_width = WIN_WIDTH / NUM_COLS as u32;
    let cell_height = WIN_HEIGHT / NUM_ROWS as u32;
    let mut draw_cell = Rect::new(0, 0, cell_width, cell_height);

    for i in 0..NUM_ROWS {
        draw_cell.y = i as i32 * cell_height as i32;
        for j in 0..NUM_COLS {
            draw_cell.x = j as i32 * cell_width as i32;

            if emu_state.screen[j][i] {
                canvas.set_draw_color(Color::RGB(0xff, 0xff, 0xff));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }

            canvas.fill_rect(draw_cell)?;
        }
    }

    canvas.present();
    Ok(())
}

/// Sdl->internal Chip8 format. Returns 0xdeadbeef on error.
/// # Arguments
/// * `kc` Raw SDL keycode.
fn sdl_keycode_to_internal(kc: Keycode) -> u32 {
    match kc {
        Keycode::Num7 => 0x1,
        Keycode::Num8 => 0x2,
        Keycode::Num9 => 0x3,
        Keycode::Num0 => 0xC,

        Keycode::U => 0x4,
        Keycode::I => 0x5,
        Keycode::O => 0x6,
        Keycode::P => 0xD,

        Keycode::J => 0x7,
        Keycode::K => 0x8,
        Keycode::L => 0x9,
        Keycode::Semicolon => 0xE,

        Keycode::N => 0xa,
        Keycode::M => 0x0,
        Keycode::Less => 0xb,
        Keycode::Greater => 0xf,
        _ => 0xdeadbeef,
    }
}

/// Run the emulation. Return string on error.
fn main() -> Result<(), String> {
    let (program, raw_program) = get_program()?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Chip8", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut emu_state = InterpreterData::new();

    let mut cur_pressed_keys = [false; 0x10];

    // Load the font into memory.
    for (i, b) in FONTSET.iter().enumerate() {
        emu_state.mem[i] = *b;
    }

    // Load the program into memory.
    for (i, b) in raw_program.iter().enumerate() {
        emu_state.mem[i + 0x200] = *b;
    }

    let mut time_passed = Duration::new(0, 0);
    let mut seconds_counter = Duration::new(0, 0);

    // Draw the blank screen once before beginning the loop.
    draw_screen(&emu_state, &mut canvas)?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'running; },
                Event::KeyDown { keycode: Some(kc), .. } => {
                    let cur_key = sdl_keycode_to_internal(kc);
                    if  cur_key != 0xdeadbeef {
                        cur_pressed_keys[cur_key as usize] = true;
                    }
                },
                Event::KeyUp { keycode: Some(kc), .. } => {
                    let cur_key = sdl_keycode_to_internal(kc);
                    if  cur_key != 0xdeadbeef {
                        cur_pressed_keys[cur_key as usize] = false;
                    }
                },
                _ => {}
            }
        }

        emulate(&program, &mut emu_state, &cur_pressed_keys);


        if time_passed > Duration::from_millis(1000 / 60) {
            time_passed = Duration::new(0, 0);
            if emu_state.draw {
                draw_screen(&emu_state, &mut canvas)?;
                emu_state.draw = false;
            }
            if emu_state.delay_timer > 0 {
                emu_state.delay_timer -= 1;
            }
            if emu_state.sound_timer > 0 {
                emu_state.sound_timer -= 1;
            }
        }
        if seconds_counter > Duration::new(1, 0) {
            seconds_counter = Duration::new(0, 0);
            //emu_state.screen = [[false; NUM_ROWS]; NUM_COLS];
        }

        // Rate of 700 instructions per second.
        const SLEEP_FOR: u64 = 1_000_000_000 / 700;
        std::thread::sleep(Duration::from_nanos(SLEEP_FOR));
        time_passed += Duration::from_nanos(SLEEP_FOR);
        seconds_counter += Duration::from_nanos(SLEEP_FOR);
    }

    Ok(())
}
