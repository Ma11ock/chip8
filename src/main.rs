extern crate sdl2;

use std::env;

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

fn main() {
    let game_file: String = get_bin_file();
    
    println!("Opening binary file {}.", game_file);
}
