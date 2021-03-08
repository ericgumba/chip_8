extern crate regex;
use regex::Regex;
use std::{convert::TryInto, fmt, usize};
 
 
pub struct Chip8{
    pub ram: [u8; 4096], 
    pub regs: [u8; 16],
    pub index_register: u16,
    pub pc: usize,
    pub rom: Vec<u8>,
    pub stack: Vec<u32>,
    pub vram: [[char; 64]; 32]
}
    fn crop_str(s: &str, n: usize) -> &str {
        let mut it = s.chars();
        for _ in 0..n {
            it.next();
        }
        it.as_str()
    }

    fn make_into_8_field( val: String) -> String{
        let mut leng = val.len();
        let mut nv: String;
        nv = val;
        while leng < 8 {
            let mut field = "0".to_string();
            field.push_str(&nv);
            nv = field;
            leng = nv.len();
            // push_str(&val);
            

        }
        nv

    }
impl Chip8 {
    pub fn new() -> Self{
        return Chip8 {
            ram: [0;4096],
            vram: [['0';64]; 32],
            regs: [0;16],
            pc: 512,
            index_register: 0,
            stack: Vec::new(), 
            rom: vec![]
        }
    }

    pub fn load(&mut self, rom:&Vec<u8>) {

        for (i, &byte) in rom.iter().enumerate(){
            self.ram[i+512] = byte
        }

        self.rom = rom.to_vec();



    }
    fn generalize_instruction(instruction: &String) -> &'static str {
        let ch = instruction.chars().next().unwrap();
         
        match ch {
            'E' => return "E0",
            '1' => return "1NNN",
            '3' => return "3XNN",
            '6' => return "6XNN",
            '7' => return "7XNN", 
            'A' => return "ANNN",
            'D' => return "DXYN",
             _ => return "FAIL"
        }; 

        
    }
    // as hexidecimal
    pub fn decode(&self, opcode: (u8,u8)) -> String { 

        let b1: u16 = (u16::from(opcode.0)) << 8 | (u16::from(opcode.1));
        format!("{:X}", b1)
    }

    pub fn still_has_more_instructions(&self) -> bool{ 

        return self.pc - 512 < self.rom.len().try_into().unwrap(); 
    }

    pub fn fetch(&mut self) -> (u8,u8) {
        let i = self.pc;
        let instructions = (self.ram[self.pc as usize], self.ram[(self.pc+1) as usize]);
        self.pc += 2;
        return  instructions;
    }

    fn convert_hex_to_u16(instruction: &str) ->  u16{

        u16::from_str_radix(instruction, 16).unwrap()

    }
    fn convert_hex_to_u8(instruction: &str) ->  u8{

        u8::from_str_radix(instruction, 16).unwrap()
 
    }

    fn set_index_register(& mut self, instruction: String){
        let reduced_instruction = crop_str(&instruction, 1);  
        self.index_register = u16::from_str_radix(reduced_instruction, 16).unwrap(); 
    }

    fn jump(&mut self, instruction: String){ 
        
        let reduced_instruction = crop_str(&instruction, 1);  
        let address = Chip8::convert_hex_to_u16(reduced_instruction); 
        self.pc = address as usize; 
    

        

    }
    pub fn clear(&mut self){
        
        for i in 0..self.vram.len() {
            for j in 0..self.vram[i].len(){ 
                self.vram[i][j] = '0'
            }
        }
    } 

    fn set_vx_register(&mut self, instruction: String){  
        println!("Setting vx register");
        
        let reduced_instruction = crop_str(&instruction, 1);  
        
        let register_to_set = reduced_instruction.chars().nth(0).unwrap();
        
        let register_to_set_dec_format = Chip8::convert_hex_to_u16(&register_to_set.to_string());
        
        let further_reduced = crop_str(reduced_instruction, 1); 
        let res = Chip8::convert_hex_to_u8(further_reduced);
        self.regs[register_to_set_dec_format as usize] = res as u8;
        println!("hello whats wrong with that {}", self.regs[register_to_set_dec_format as usize])

    }
    fn add_to_vx_register(&mut self, instruction: String){  
        println!("adding vx register"); 
        let reduced_instruction = crop_str(&instruction, 1);   
        let register_to_set = reduced_instruction.chars().nth(0).unwrap(); 
        let register_to_set_dec_format = Chip8::convert_hex_to_u16(&register_to_set.to_string()); 
        let further_reduced = crop_str(reduced_instruction, 1); 
        let res = Chip8::convert_hex_to_u8(further_reduced);
        println!("regist to set: {}, further reduced: {}, res to add: {}", register_to_set_dec_format, further_reduced, res);
        self.regs[register_to_set_dec_format as usize] += res as u8 
    }
 

    // dxyn
    pub fn draw(& mut self, instruction: String){
        
        println!("DRAWING");
        let reduced_instruction = crop_str(&instruction, 1);
        let vx = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(0).unwrap().to_string());
        let vy = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(1).unwrap().to_string());
        let x_coord: usize = self.regs[vx as usize].into();
        let y_coord: usize = self.regs[vy as usize].into();
        
        let bytes_to_read = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(2).unwrap().to_string()); 
        println!("X_coord:{} , y_coord: {}", x_coord, y_coord);
        for i in 0..bytes_to_read{ 
            let mut sprite_template = format!("{:b}",self.ram[ self.index_register as usize + i as usize ]); 
            
            sprite_template = make_into_8_field(sprite_template);
            println!("sprite template {}", sprite_template);
            println!("original {}", self.ram[ self.index_register as usize + i as usize ]);
            
            
            for j in 0..sprite_template.len(){
                if sprite_template.chars().nth(j).unwrap() == '1' {
                    if self.vram[y_coord+i as usize][x_coord+j] == '1'{
                        
                        self.vram[y_coord+i as usize][x_coord+j] = '0';
                    } else {
                        self.vram[y_coord+i as usize][x_coord+j] = '1';
                    }
                    // println!("X_coord:{} , y_coord: {}", x_coord, y_coord);
                } // why no deal with 0 case? Because if it's an identity element.

            }
        } 
    } 

    pub fn can_skip(&mut self, instruction: String, reg_value: u8) -> bool{

        let op_code = Chip8::convert_hex_to_u8(&instruction.chars().nth(0).unwrap().to_string());
        let nn_str = crop_str(&instruction, 2);
        let nn = Chip8::convert_hex_to_u8(nn_str);
        if op_code == 3{
            reg_value == nn.try_into().unwrap()  
        } else if op_code == 4 {
            reg_value != nn.try_into().unwrap()  
        } else { 
            
            println!("ERROR OCCURRED!");
            false
        }
    }
    
    // 3XNN will skip one instruction if the value in VX is equal to NN, and 4XNN will skip if they are not equal.
    pub fn skip(&mut self, instruction: String){

        let register_index_hex = &instruction.chars().nth(1).unwrap().to_string();
        let register_index_dec = Chip8::convert_hex_to_u8(register_index_hex);
        let reg_value = self.regs[register_index_dec as usize]; 
        if self.can_skip(instruction, reg_value) {
            self.pc += 2;
        }  
    }
    
    pub fn execute(&mut self, instruction: String){ 

        println!("executing : {}", instruction);
        
        let generalized_instruction = Chip8::generalize_instruction(&instruction); 
        

        // 00E0 (clear screen)
        // 1NNN (jump)
        // 6XNN (set register VX)
        // 7XNN (add value to register VX)
        // ANNN (set index register I)
        // DXYN (display/draw)


        match generalized_instruction {
            "E0" => self.clear(),
            "1NNN"   => self.jump(instruction),
            "6XNN"   => self.set_vx_register(instruction), // clear
            "7XNN"   => self.add_to_vx_register(instruction), // set vx reg
            "ANNN"   => self.set_index_register(instruction),
            "DXYN"   => self.draw(instruction), // draw 
            "3XNN"   => self.skip(instruction),
            _        => println!("No gift? Oh well. {}", instruction),
        }
        
    }
}
// TODO: implement this 


