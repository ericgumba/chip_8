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
mod audio;

const CHIP8_WIDTH: usize = 32;
const CHIP8_HEIGHT: usize = 64;
const CHIP8_RAM: usize = 4096;

fn main() {
    // Create a path to the desired file 
    let path = Path::new("./Pong.ch8");
    let display = path.display();  
 
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
 
    let x =read_file("Pong.ch8".to_string());

    let sdl_context = sdl2::init().unwrap();
    
    
    let mut my_chip8 = chip8::Chip8::new();
    let mut drawer = drawer::Drawer::new(&sdl_context);
    let mut keyboard = keyboard::Keyboard::new(&sdl_context);
    my_chip8.load(&x); 
    
     
    while my_chip8.still_has_more_instructions() { 
        
        keyboard.poll_keyboard(); 

        if keyboard.quit {
             break;
        }
        // if keyboard.p {
        my_chip8.set_keyboard(keyboard.keys_pressed); 
        let opcode = my_chip8.fetch(); 
        let instruction = my_chip8.decode(opcode);
        my_chip8.delay_timer.tick();  
        

        if my_chip8.sound_timer.val > 0{ audio::play(&sdl_context); }
        my_chip8.sound_timer.tick();  
        my_chip8.execute(instruction); 
        if my_chip8.vram_changed{
        drawer.draw(my_chip8.vram); 
        }
        my_chip8.vram_changed = false;
        std::thread::sleep(Duration::from_millis(2)); 
        // }
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
 