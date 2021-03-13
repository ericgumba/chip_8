#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

use sdl2::{event::Event, keyboard::Keycode}; 
mod drawer;
mod keyboard;
mod chip8;
mod chip8_test;

const CHIP8_WIDTH: usize = 32;
const CHIP8_HEIGHT: usize = 64;
const CHIP8_RAM: usize = 4096;

fn main() {
    // Create a path to the desired file 
    let path = Path::new("./IBM Logo.ch8");
    let display = path.display();  
 
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
 
    let x =read_file("IBM Logo.ch8".to_string());

    let sdl_context = sdl2::init().unwrap();
     
    // let mut event_pumcp = sdl_context.event_pump().unwrap();
    let mut my_chip8 = chip8::Chip8::new();
    let mut drawer = drawer::Drawer::new(&sdl_context);
    let mut keyboard = keyboard::Keyboard::new(&sdl_context);
    my_chip8.load(&x); 

    let mut zx:u16 = 0; 

    // my_chip8.clear();
    // drawer.clear();
    
    while my_chip8.still_has_more_instructions() {
        keyboard.poll_keyboard(); 
        my_chip8.set_keyboard(keyboard.keys_pressed); 
        let opcode = my_chip8.fetch(); 
        let instruction = my_chip8.decode(opcode);  
        my_chip8.execute(instruction); 
        drawer.draw(my_chip8.vram); 
        let mut i = 0; 
        i = (i + 1) % 255;  
        zx += 1;
        if zx == 1000{
            break;
        }
    }
    
}

fn read_file(mut file_name: String) -> Vec<u8> {
    file_name = file_name.replace("/", "");
    if file_name.is_empty() {
        file_name = String::from("index.html");
    }

    let path = Path::new(&file_name);
    if !path.exists() {
        return String::from("Not Found!").into();
    }
    let mut file_content = Vec::new();
    let mut file = File::open(&file_name).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}
 