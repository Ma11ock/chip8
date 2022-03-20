extern crate sdl2;

use std::env;
use std::fs;

// Includes all data needed by the interpreter.
#[derive(Copy, Clone)]
struct InterpreterData {
    // V registers. 16 of them, general purpose, 8 bits.
    v: [u8; 16],
    // I register, 16 bits.
    i: u16,
    // Program counter, 16 bits.
    pc: u16,
    // Stack pointer, 16 bits.
    sp: u8,
    // The call stack, 16 levels of 16 bits each.
    stack: [u16; 16],
    // Delay timer @ 60Hz, 8 bits.
    delay_timer: u8,
    // Sound timer @ 60Hz, 8 bits.
    sound_timer: u8,
}

impl InterpreterData {
    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn push_stack(&mut self) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
    }

    fn get_register(&self, reg: u8) -> u8 {
        self.v[reg]
    }
}

union Memory {
    // Entire memory of the emulator.
    mem: [u8; 4096],
    data: InterpreterData,
}

fn get_last_3_nibbles(n: u16) {
    n & 0x0FFFu16
}

fn get_last_2_nibbles(n: u16) {
    n & 0x00FFu16
}

fn get_third_nibble(n: u16) {
    n & 0x0F00
}

fn get_second_nibble(n: u16) {
    n & 0x00F0
}

fn emulate(file: &Vec<u16>) -> Result<(), String> {
    let mut emu_state = Memory{ mem: [0u8; 4096] };
    fn increment_pc(n: Option<u16>) -> u16 {
        emu_state.data.pc + n.unwrap_or(1)
    }
    // Read the file.
    for i in 0..file.len() {
        // Check first nibble
        let new_pc = match file[i] >> 12 {
            0 => {
                if file[i + 1] == 0xE0 {
                    // Clear the display.
                    // TODO 
                    increment_pc(None)
                } else if file[i + 1] == 0xEE {
                    // Set PC to to stack[sp], decrement sp.
                    unsafe { emu_state.data.pop_stack() }
                }
            },
            // Set PC to bottom three nibbles.
            1 => {
                unsafe { get_last_3_nibbles(file[i]) }
            },
            // Function call at bottom three nibbles.
            2 => {
                unsafe {
                    emu_state.data.push_stack();
                    get_last_3_nibbles(file[i])
                }
            },
            // Skip next instruction if the bottom byte is equal to the value
            // in V[first nibble].
            3 => {
                let kk = get_last_2_nibbles();
                unsafe {
                    if emu_state.data.get_register(get_third_nibble(file[i])) == kk {
                        increment_pc(Some(2))
                    } else {
                        increment_pc(None)
                    }
                }
            },
            // Skip next instruction if V[third nibble] == bottom byte.
            4 => {
                let kk = get_last_2_nibbles();
                unsafe {
                    if emu_state.data.get_register(get_third_nibble(file[i])) != kk {
                        increment_pc(Some(2))
                    } else {
                        increment_pc(None)
                    }
                }
            },
            // If V[third nibble] == V[second nibble] then skip next instruction.
            5 => {
                unsafe {
                    if emu_state.data.get_register(get_third_nibble(file[i])) ==
                        emu_state.data.get_register(get_second_nibble(file[i])) {
                            increment_pc(Some(2))
                        } else {
                            increment_pc(None)
                        }
                }
            },
            // Load bottom byte into V[third nibble].
            6 => {
                let kk = get_last_2_nibbles();
            },
            _ => return Err(format!("Invalid instruction at position {i}: 0x{:x}.", file[i])),
        }
    }
    Ok(())
}

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

fn main() -> Result<(), String> {
    let game_file: String = get_bin_file();
    println!("Opening binary file {}.", game_file);

    let raw_program = match fs::read(game_file) {
        Ok(p) => p,
        Err(e) => return Err(e.to_string()),
    };

    let program = match convert_bin_format(&raw_program) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    match emulate(&program) {
        Err(e) => return Err(e),
        Ok(_) => {},
    }

    Ok(())
}
