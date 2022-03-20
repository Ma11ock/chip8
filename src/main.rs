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
    // Memory.
    mem: [u8; 4096],
}

impl InterpreterData {
    pub fn new() -> Self {
        Self { v: [0; 16],
               i: 0,
               pc: 0,
               sp: 0,
               stack: [0; 16],
               delay_timer: 0,
               sound_timer: 0,
               mem: [0; 4096] }
    }

    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn push_stack(&mut self) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
    }

    fn get_register(&self, reg: u8) -> u8 {
        self.v[reg as usize]
    }

    fn increment_pc(&self, amount: u16) -> u16 {
        self.pc + amount
    }
}

fn get_last_3_nibbles(n: u16) -> u16 {
    n & 0x0FFFu16
}

fn get_last_2_nibbles(n: u16) -> u8 {
    (n & 0x00FFu16) as u8
}

fn get_third_nibble(n: u16) -> u8 {
    ((n & 0x0F00u16) >> 8) as u8
}

fn get_second_nibble(n: u16) -> u8 {
    ((n & 0x00F0) >> 4) as u8
}

fn invalid_instruction_message(index: usize, what: u16) -> String {
    format!("Invalid instruction at position {}: 0x{:x}.", index, what)
}

fn emulate(file: &Vec<u16>) -> Result<(), String> {
    let mut emu_state = InterpreterData::new();
    
    // Read the file.
    for i in 0..file.len() {
        let cur_instruction = file[i];
        // Check first nibble
        emu_state.pc = match cur_instruction >> 12 {
            0 => {
                if get_last_2_nibbles(cur_instruction) == 0xE0 {
                    // Clear the display.
                    // TODO 
                    emu_state.increment_pc(1)
                } else if get_last_2_nibbles(cur_instruction) == 0xEE {
                    // Set PC to to stack[sp], decrement sp.
                    emu_state.pop_stack() 
                } else {
                    return Err(invalid_instruction_message(i, cur_instruction));
                }
            },
            // Set PC to bottom three nibbles.
            1 => {
                get_last_3_nibbles(cur_instruction) 
            },
            // Function call at bottom three nibbles.
            2 => {
                emu_state.push_stack();
                get_last_3_nibbles(cur_instruction)
            },
            // Skip next instruction if the bottom byte is equal to the value
            // in V[first nibble].
            3 => {
                let kk = get_last_2_nibbles(cur_instruction) as u8;
                if emu_state.get_register(get_third_nibble(cur_instruction)) == kk {
                    emu_state.increment_pc(2)
                } else {
                    emu_state.increment_pc(1)
                }
            },
            // Skip next instruction if V[third nibble] == bottom byte.
            4 => {
                let kk = get_last_2_nibbles(cur_instruction);
                if emu_state.get_register(get_third_nibble(cur_instruction)) != kk {
                    emu_state.increment_pc(2)
                } else {
                    emu_state.increment_pc(1)
                }
            },
            // If V[third nibble] == V[second nibble] then skip next instruction.
            5 => {
                if emu_state.get_register(get_third_nibble(cur_instruction)) ==
                    emu_state.get_register(get_second_nibble(cur_instruction)) {
                        emu_state.increment_pc(2)
                    } else {
                        emu_state.increment_pc(1)
                    }
            },
            // Load bottom byte into V[third nibble].
            _ => return Err(invalid_instruction_message(i, cur_instruction)),
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
