extern crate regex;
use regex::Regex;
use std::fmt;  
 
pub struct Chip8{
    pub ram: [u32; 4096],
    pub rom: Vec<u8>,
    pub regs: [i8; 16],
    pub pc: i64,
    pub stack: Vec<i32>
}


impl Chip8 {
    pub fn new(rom:Vec<u8>) -> Chip8{
        return Chip8 {
            ram: [0;4096],
            regs: [0;16],
            pc: 0,
            stack: Vec::new(),
            rom: rom
        }
    }
    fn generalize_instruction(instruction: &String) -> &'static str {
        let ch = instruction.chars().next().unwrap();
         
        match ch {
            'E' => return "E0",
            '1' => return "1NNN",
            '6' => return "6XNN",
            '7' => return "7XNN",
            'A' => return "ANNN",
            'D' => return "DXYN",
             _ => return "FAIL"
        }; 

        
    }
    pub fn decode(&self, opcode: (u8,u8)) -> String { 

        let b1: u16 = (u16::from(opcode.0)) << 8 | (u16::from(opcode.1));
        format!("{:X}", b1)
    }

    pub fn fetch(&mut self) -> (u8,u8) {
        let i = self.pc;
        let instructions = (self.rom[self.pc as usize], self.rom[(self.pc+1) as usize]);
        self.pc += 2;
        return  instructions;
    }

    pub fn execute(&self, instruction: String){
        let generalized_instruction = Chip8::generalize_instruction(&instruction);
        println!("test: {}", generalized_instruction);

        match generalized_instruction {
            "E0" => println!("Yuck! I'm putting this snake back in the forest."),
            "1NNN"   => {  
            },
            "6XNN"   => println!("6xnn"),
            "7XNN"   => println!("7XNN"),
            "ANNN"   => println!("ANNN => {}", instruction),
            "DXYN"   => println!("? How nice."),
            _        => println!("No gift? Oh well."),
        }
        
    }
}
// TODO: implement this 



