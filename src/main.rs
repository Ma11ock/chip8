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
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
        self.pc
    }
}

union Memory {
    // Entire memory of the emulator.
    mem: [u8; 4096],
    data: InterpreterData,
}

fn emulate(file: &Vec<u16>) -> Result<(), String> {
    let mut emu_state = Memory{ mem: [0u8; 4096] };
    // Read the file.
    for i in 0..file.len() {
        // Check first nibble
        match file[i] >> 12 {
            0 => {
                if file[i + 1] == 0xE0 {
                    // Clear the display.
                    // TODO 
                } else if file[i + 1] == 0xEE {
                    // Set PC to to stack[sp], decrement sp.
                    unsafe { emu_state.data.pop_stack() };
                }
            },
            // Set PC to bottom three nibbles.
            1 => {

            },
            2 => {
                
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
