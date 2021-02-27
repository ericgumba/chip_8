#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::time::Duration;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*; 
mod drawer;
mod chip8;

fn main() {
    // Create a path to the desired file 
    let path = Path::new("./IBM Logo.ch8");
    let display = path.display();  

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut my_drawer = drawer::Drawer::new();

    my_drawer.draw();
    my_drawer.clear();
 
    let x =read_file("Chip8 Picture.ch8".to_string());
 
    let mut my_chip8 = chip8::Chip8::new(x);
  
    let opcode = my_chip8.fetch(); 
    let instruction = my_chip8.decode(opcode); 
    println!("my inst: {}", instruction);
    my_chip8.execute(instruction);


    let opcode = my_chip8.fetch(); 
    let instruction = my_chip8.decode(opcode); 
    println!("my inst: {}", instruction);
    my_chip8.execute(instruction);



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
 