extern crate regex;
use rand::Rng;
use regex::Regex;
use std::{convert::TryInto, fmt, usize};

pub fn get_value_of_key(i:u8) -> u8{

    match i {
        0 => 0x1,
        1 => 0x2,
        2 => 0x3,
        3 => 0xC,
        4 => 0x4,
        5 => 0x5,
        6 => 0x6,
        7 => 0xD,
        8 => 0x7,
        9 => 0x8,
        10 => 0x9,
        11 => 0xE,
        12 => 0xA,
        13 => 0x0,
        14 => 0xB,
        15 => 0xF,
        _ => 0xF1
    }

}
pub struct Timer{
    pub val: u32
}

impl Timer {
    pub fn tick(&mut self){
        if self.val > 0  {
            self.val -= 1;
        }
    }
} 
pub struct Chip8{
    pub ram: [u8; 4096], 
    pub regs: [u8; 16],
    pub index_register: u16,
    pub pc: usize,
    pub font_locations: [u8; 16],
    pub rom: Vec<u8>,
    pub stack: Vec<u32>,
    pub vram: [[char; 64]; 32],
    pub vram_changed: bool,
    pub keyboard: [bool; 16],
    pub delay_timer: Timer,
    pub sound_timer: Timer,
}

pub fn convert_vx_value_into_keyboard_index_value(hex: String) -> usize {
    match &hex[..] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        "C" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "D" => 7,
        "7" => 8,
        "8" => 9,
        "9" => 10,
        "E" => 11,
        "A" => 12,
        "0" => 13,
        "B" => 14,
        "F" => 15,
        _ => usize::MAX
    }

}
    fn crop_str(s: &str, n: usize) -> &str {
        let mut it = s.chars();
        for _ in 0..n {
            it.next();
        }
        it.as_str()
    }

    
    fn make_into_n_field(n:usize, val: String) -> String{
        let mut leng = val.len();
        let mut nv: String;
        nv = val;
        while leng < n {
            let mut field = "0".to_string();
            field.push_str(&nv);
            nv = field;
            leng = nv.len();
            // push_str(&val);
            
            
        }
        nv 
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
                rom: vec![],
                font_locations:[0;16],
                keyboard: [false; 16],
                vram_changed: false,
                delay_timer: Timer { val: 0 },
                sound_timer: Timer { val: 0 },
            }
        }
        pub fn set_keyboard(&mut self, keyboard: [bool;16]){
            self.keyboard = keyboard
        }

        pub fn load_font(&mut self){
            let font = [
                0xF0, 0x90, 0x90, 0x90, 0xF0,  
                0x20, 0x60, 0x20, 0x20, 0x70,  
                0xF0, 0x10, 0xF0, 0x80, 0xF0,  
                0xF0, 0x10, 0xF0, 0x10, 0xF0,  
                0x90, 0x90, 0xF0, 0x10, 0x10,  
                0xF0, 0x80, 0xF0, 0x10, 0xF0,  
                0xF0, 0x80, 0xF0, 0x90, 0xF0,  
                0xF0, 0x10, 0x20, 0x40, 0x40,  
                0xF0, 0x90, 0xF0, 0x90, 0xF0,  
                0xF0, 0x90, 0xF0, 0x10, 0xF0,  
                0xF0, 0x90, 0xF0, 0x90, 0x90, 
                0xE0, 0x90, 0xE0, 0x90, 0xE0,  
                0xF0, 0x80, 0x80, 0x80, 0xF0,  
                0xE0, 0x90, 0x90, 0x90, 0xE0, 
                0xF0, 0x80, 0xF0, 0x80, 0xF0, 
                0xF0, 0x80, 0xF0, 0x80, 0x80
                ];

            for (i, val) in font.iter().enumerate(){
                self.ram[i] = *val;
            }

            self.font_locations =[0,5,10,15,20,25,30,35,40,45,50,55,60,65,70,75];



        }
        
        pub fn load(&mut self, rom:&Vec<u8>) {

            self.load_font();
            
            for (i, &byte) in rom.iter().enumerate(){
                self.ram[i+512] = byte
            }
            
            self.rom = rom.to_vec();
            


    }
    fn generalize_instruction(instruction: &mut String) -> &'static str {

        let mut formated_instruction = instruction.to_string(); 
 
        if instruction.len() < 4 {
            formated_instruction = make_into_n_field(4, instruction.to_string());
        }
 
        let ch = formated_instruction.chars().next().unwrap();
        let ch2 = formated_instruction.chars().nth(1).unwrap(); 
        let ch3 = formated_instruction.chars().nth(2).unwrap();
        let ch4 = formated_instruction.chars().nth(3).unwrap(); 

        if ch == 'E' && ch4 == '1'{ 
            return "EXA1"
        }
        if ch == 'F' && ch4 == 'A'{ 
            return "FX0A"
        }
        if ch == 'F' && ch4 == 'E'{ 
            return "FX1E"
        }
        if ch == 'F' && ch4 == '3'{ 
            return "FX33"
        }
        if ch == 'F' && ch4 == '5' && ch3 == '1'{ 
            return "FX15"
        }
        if ch == 'F' && ch4 == '5' && ch3 == '5'{ 
            return "FX55"
        }
        if ch == 'F' && ch4 == '5' && ch3 == '6'{ 
            return "FX65"
        }
        if ch == 'F' && ch4 == '8'{ 
            return "FX18"
        }
        if ch == 'F' && ch4 == '9'{ 
            return "FX29"
        }
        if ch == '8' && ch4 == '2'{ 
            return "8XY2"
        }
        if ch == '8' && ch4 == '3'{ 
            return "8XY3"
        }
        if ch == '8' && ch4 == '4'{ 
            return "8XY4"
        }
        if ch == '8' && ch4 == '5'{ 
            return "8XY5"
        }
        if ch == '8' && ch4 == '6'{ 
            return "8XY6"
        }
        if ch == '8' && ch4 == '7'{ 
            return "8XY7"
        }
        if ch == '8' && ch4 == 'E'{ 
            return "8XYE"
        }
        if ch == '0' && ch4 == 'E' {
            return "00EE"
        }

        match ch {
            '0' => return "00E0",
            '1' => return "1NNN",
            '2' => return "2NNN",
            '3' => return "3XNN",
            '4' => return "4XNN",
            '5' => return "5BC0",
            '6' => return "6XNN",
            '7' => return "7XNN", 
            '8' => return "8XY1", 
            '9' => return "9XY0", 
            'A' => return "ANNN",
            'C' => return "CXNN",
            'D' => return "DXYN",
            'E' => return "EX9E",
            'F' => return "FX07",
             _ => return "FAIL"
        }; 

        
    }
    // EX9E
    pub fn ex9e(&mut self, instruction:String){ 
        let vx = self.get_reg_value_from_hex( instruction.chars().nth(1).unwrap() ) ; 
        let vx_in_hex = format!("{:X}", vx);
        let keyboard_index = convert_vx_value_into_keyboard_index_value( vx_in_hex ); 
        if self.keyboard[keyboard_index] {
            self.pc += 2;
        } 
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
        self.vram_changed = true;
    } 

    fn set_vx_register(&mut self, instruction: String){   
        
        let reduced_instruction = crop_str(&instruction, 1);  
        let register_to_set = reduced_instruction.chars().nth(0).unwrap();
        let register_to_set_dec_format = Chip8::convert_hex_to_u16(&register_to_set.to_string());
        let further_reduced = crop_str(reduced_instruction, 1); 
        let res = Chip8::convert_hex_to_u16(further_reduced);
        self.regs[register_to_set_dec_format as usize] = res as u8; 
        
        
        

    }


        // 7xkk - ADD Vx, byte
        // Set Vx = Vx + kk.

        // Adds the value kk to the value of register Vx, then stores the result in Vx.

    fn add_to_vx_register(&mut self, instruction: String){   
        let reduced_instruction = crop_str(&instruction, 1);   
        let register_to_set = reduced_instruction.chars().nth(0).unwrap(); 
        let register_to_set_dec_format = Chip8::convert_hex_to_u16(&register_to_set.to_string()); 
        let further_reduced = crop_str(reduced_instruction, 1); 
        let res = Chip8::convert_hex_to_u16(further_reduced);  
        let real_res2 = self.regs[register_to_set_dec_format as usize] as u16;
        let f = real_res2 + res;
        self.regs[register_to_set_dec_format as usize] = f as u8;
    }
 

    // dxyn
    pub fn draw(& mut self, instruction: String){
        self.vram_changed = true;
        let reduced_instruction = crop_str(&instruction, 1);
         
        let vx = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(0).unwrap().to_string());
        let vy = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(1).unwrap().to_string());
        let x_coord: usize = self.regs[vx as usize].into();
        let y_coord: usize = self.regs[vy as usize].into();
        
        let bytes_to_read = Chip8::convert_hex_to_u8(&reduced_instruction.chars().nth(2).unwrap().to_string());  
        let mut sprite_deleted = false;
        for i in 0..bytes_to_read{ 
            let mut sprite_template = format!("{:b}",self.ram[ self.index_register as usize + (i) as usize ]); 
            
            sprite_template = make_into_8_field(sprite_template); 
             
            for j in 0..sprite_template.len(){
                if sprite_template.chars().nth(j).unwrap() == '1' {
                    let y = (y_coord+i as usize) % 32;
                    let x = (x_coord+j as usize) % 64; 

                    if self.vram[y][x] == '1'{
                        
                        self.vram[y][x] = '0';
                        sprite_deleted = true;

                    } else {
                        self.vram[y][x] = '1';
                    }
                    // println!("X_coord:{} , y_coord: {}", x_coord, y_coord);
                } // why no deal with 0 case? Because if it's an identity element.

            }
        }
        if sprite_deleted{
            self.regs[0xf] = 1;
        }

        else {
            self.regs[0xf] = 0;
        }
    } 
    

    pub fn can_skip(&mut self, instruction: String, reg_value: u8) -> bool{

        let op_code = Chip8::convert_hex_to_u8(&instruction.chars().nth(0).unwrap().to_string());
        let nn_str = crop_str(&instruction, 2);
        let nn = Chip8::convert_hex_to_u8(nn_str);
        if op_code == 3{
                return reg_value == nn.try_into().unwrap()  

        } else if op_code == 4 {
                return reg_value != nn.try_into().unwrap()  
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

    pub fn get_reg_value_from_hex(&mut self, hex: char) -> u8{

        self.regs[ Chip8::convert_hex_to_u8( &hex.to_string() ) as usize ]

    }


    // 8xy1 - OR Vx, Vy
    pub fn or_vx_with_vy(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        self.regs[vx_key as usize] |= self.regs[vy_key as usize];
    }
    pub fn and_vx_with_vy(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        self.regs[vx_key as usize] &= self.regs[vy_key as usize];
    }
    pub fn xor_vx_with_vy(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        self.regs[vx_key as usize] ^= self.regs[vy_key as usize];
    }
    pub fn add_vx_with_vy(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        let res: u16 = (self.regs[vx_key as usize] as u16+self.regs[vy_key as usize] as u16).into();
 
        if res > 255 { 
            self.regs[0xF] = 1
        } else {
            self.regs[0xF] = 0

        }
        self.regs[vx_key as usize] = res as u8;



    }
    pub fn sub_vx_with_vy(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        let vx = self.regs[vx_key as usize];
        let vy = self.regs[vy_key as usize];
        let res:u8;
 
        self.regs[0xf] = if vx > vy {1} else {0};  
        
        res = vx.wrapping_sub(vy);
        self.regs[vx_key as usize] = res; 
    }
    pub fn sub_vy_with_vx(&mut self, instruction: String){ 

        println!("DEBUG CANDIDATE"); 
        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string());
        let vy_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(2).unwrap().to_string());
        let vx = self.regs[vx_key as usize];
        let vy = self.regs[vy_key as usize];
        let res:u8;
 
        self.regs[0xf] = if vy > vx {1} else {0};  
        
        res = vy.wrapping_sub(vx);
        self.regs[vx_key as usize] = res; 
    }
    pub fn div_vx_by_two(&mut self, instruction: String){ 
        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string()); 
        let vx = self.regs[vx_key as usize];   
        self.regs[0xf] = if vx & 1 == 1 {1} else { 0 };
        self.regs[vx_key as usize] = vx >> 1; 
    }
    
    pub fn mult_vx_by_two(&mut self, instruction: String){ 

        let vx_key = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string()); 
        let vx = self.regs[vx_key as usize];   

        self.regs[0xf] = if vx & 1 == 1 {1} else { 0 };

        self.regs[vx_key as usize] = vx << 1; 
    }
    pub fn ret(&mut self){  
        self.pc = self.stack.pop().unwrap() as usize;
    }
    pub fn subroutine(&mut self, instruction: String){  
        self.stack.push((self.pc) as u32); 

        let reduced_instructions = crop_str(&instruction, 1);

        self.pc = Chip8::convert_hex_to_u16(reduced_instructions) as usize
    }
    
    
    pub fn skip_if_eq(&mut self, instruction: String){  
        let x_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(1).unwrap().to_string() ) as usize;
        let y_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(2).unwrap().to_string() ) as usize;
        
        self.pc = if self.regs[x_reg] == self.regs[y_reg] { self.pc + 2 } else { self.pc };
    }
    // 4xnn
    pub fn skip_if_neq(&mut self, instruction: String){  
        let x_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(1).unwrap().to_string() ) as usize; 
        let nn = Chip8::convert_hex_to_u16( crop_str(&instruction, 2) );
          
            self.pc = if self.regs[x_reg] == nn as u8 { self.pc } else{ self.pc+2 } 
    }
    pub fn skip_if_vy_neq_vx(&mut self, instruction: String){ 
        let x_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(1).unwrap().to_string() ) as usize; 
        let y_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(2).unwrap().to_string() ) as usize;
        println!("here {}", self.pc);
        self.pc = if self.regs[x_reg] == self.regs[y_reg] { self.pc } else{ self.pc+2 };
        println!("again {}", self.pc);
    }
    pub fn rand_and_nn(&mut self, instruction: String){  
        let x_reg = Chip8::convert_hex_to_u16(&instruction.chars().nth(1).unwrap().to_string() ) as usize; 
        let mut rng = rand::thread_rng(); 
        let rand: u8 = rng.gen(); 
        let nn = Chip8::convert_hex_to_u8(crop_str(&instruction, 2));
        self.regs[x_reg] = nn & rand; 
    }
    pub fn exa1(&mut self, instruction: String){  
        let vx = self.get_reg_value_from_hex( instruction.chars().nth(1).unwrap() ) ; 
        let vx_in_hex = format!("{:X}", vx);
        let keyboard_index = convert_vx_value_into_keyboard_index_value( vx_in_hex ); 
        if !self.keyboard[keyboard_index] {
            self.pc += 2;
        } 
    }
    pub fn fx07(&mut self, instruction: String){  
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        self.regs[vx_index as usize] = self.delay_timer.val as u8; 
    }
    pub fn fx15(&mut self, instruction: String){ 
        // no 
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        
        self.delay_timer.val  = (self.regs[vx_index as usize] as u8).into() 
    }
    
    pub fn fx18(&mut self, instruction: String){  
        // no
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        
        self.sound_timer.val  = (self.regs[vx_index as usize] as u8).into() 
    }
    
    
    // Fx0A - LD Vx, K
    // Wait for a key press, store the value of the key in Vx.
    
    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
    
    pub fn fx0a(&mut self, instruction: String){   
        println!("helo");
        // no
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        let mut key_is_pressed = false;
        for i in 0..self.keyboard.len(){ 
            if self.keyboard[i] {
                self.regs[vx_index as usize] = get_value_of_key(i as u8);
                key_is_pressed = true;
                break;
            }
        }
        if !key_is_pressed {
            self.pc -= 2;
            if self.sound_timer.val != 0 {
            self.sound_timer.val += 1;
        }
        if self.delay_timer.val != 0 {
            self.delay_timer.val += 1;
        }
        
    }
    
    
}
// yes
pub fn fx1e(&mut self, instruction: String){   
    let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        self.index_register += self.regs[vx_index as usize] as u16;
        self.regs[0x0f] = if self.index_register > 0x0F00 { 1 } else { 0 };
    }
    // no
    pub fn fx29(&mut self, instruction: String){   
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        self.index_register = self.font_locations[ self.regs[vx_index as usize] as usize ] as u16;

    }
    // no
    pub fn fx55(&mut self, instruction: String){   
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );  
        
        for i in 0..vx_index+1{
            self.ram[self.index_register as usize + i as usize] = self.regs[i as usize];
        }
        
    }
    
    
    /* 

Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.

    */
    // no
    pub fn fx65(&mut self, instruction: String){    
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );  
        
        for i in 0..vx_index+1 {
            
            self.regs[i as usize] = self.ram[self.index_register as usize + i as usize];
        }
    // no
    }
    pub fn fx33(&mut self, instruction: String){   
        let vx_index = Chip8::convert_hex_to_u8(&instruction.chars().nth(1).unwrap().to_string() );
        let vx = self.regs[vx_index as usize];
        let vx_string = vx.to_string();
        if vx_string.len() == 3{
            let hundred = vx_string.chars().nth(0).unwrap() as u8 - 0x30;
            let ten = vx_string.chars().nth(1).unwrap() as u8 - 0x30;
            let one = vx_string.chars().nth(2).unwrap() as u8 - 0x30;
            self.ram[self.index_register as usize] = hundred;
            self.ram[(self.index_register+1) as usize] = ten;
            self.ram[(self.index_register+2) as usize] = one;
        }
        if vx_string.len() == 2{
            
            let ten = vx_string.chars().nth(0).unwrap() as u8 - 0x30;
            let one = vx_string.chars().nth(1).unwrap() as u8 - 0x30;  
            self.ram[(self.index_register) as usize] = ten;
            self.ram[(self.index_register+1) as usize] = one;
        }
        if vx_string.len() == 1{
            let one = vx_string.chars().nth(0).unwrap() as u8 - 0x30; 
            self.ram[self.index_register as usize] = one; 
            
        }
        
    }
    
    pub fn execute(&mut self, instruction: String){ 
        
        
        let generalized_instruction = Chip8::generalize_instruction(&mut instruction.to_string()); 
         
        // 00E0 (clear screen)
        // 1NNN (jump)
        // 6XNN (set register VX)
        // 7XNN (add value to register VX)
        // ANNN (set index register I)
        // 8xy1 - OR Vx, Vy
        // Set Vx = Vx OR Vy.
        // DXYN (display/draw) 
        // println!("instruction?? {}", instruction);
        // println!("FML {}", generalized_instruction);


        match generalized_instruction {
            "00E0" => self.clear(),
            "00EE" => self.ret(),
            "1NNN"   => self.jump(instruction),
            "2NNN"   => self.subroutine(instruction),
            "4XNN"   => self.skip_if_neq(instruction),
            "5BC0"   => self.skip_if_eq(instruction),
            "6XNN"   => self.set_vx_register(instruction), // clear
            "7XNN"   => self.add_to_vx_register(instruction), // set vx reg
            "8XY1"   => self.or_vx_with_vy(instruction), // set vx reg
            "8XY2"   => self.and_vx_with_vy(instruction), // set vx reg
            "8XY3"   => self.xor_vx_with_vy(instruction), // set vx reg
            "8XY4"   => self.add_vx_with_vy(instruction), // set vx reg
            "8XY5"   => self.sub_vx_with_vy(instruction), // set vx reg
            "8XY6"   => self.div_vx_by_two(instruction), // set vx reg
            "8XY7"   => self.sub_vy_with_vx(instruction), // set vx reg
            "8XYE"   => self.mult_vx_by_two(instruction), // set vx reg
            "9XY0"   => self.skip_if_vy_neq_vx(instruction), // set vx reg
            "ANNN"   => self.set_index_register(instruction),
            "CXNN"   => self.rand_and_nn(instruction),
            "DXYN"   => self.draw(instruction), // draw 
            "EX9E"   => self.ex9e(instruction),  
            "EXA1"   => self.exa1(instruction),  
            "FX07"   => self.fx07(instruction),  
            "FX0A"   => self.fx0a(instruction),  
            "FX15"   => self.fx15(instruction),  
            "FX18"   => self.fx18(instruction),  
            "FX1E"   => self.fx1e(instruction),  
            "FX29"   => self.fx29(instruction),  
            "FX33"   => self.fx33(instruction),  
            "FX55"   => self.fx55(instruction),  
            "FX65"   => self.fx65(instruction),  
            "3XNN"   => self.skip(instruction),
            _        => println!("No gift? Oh well. {}", instruction),
        }
        
    }
}
// TODO: implement this 


