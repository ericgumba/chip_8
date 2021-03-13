use crate::chip8::Chip8;

use super::*;

fn build_chip8() -> Chip8 {
    let rom = vec![2,3,5,7];
    let mut c = chip8::Chip8::new();
    
    c.load(&rom);
    c
    
    
}
#[cfg(test)] 
mod tests {
    use crate::chip8::{self, Chip8};
    
    use super::build_chip8;

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


// Fx0A - LD Vx, K
// Wait for a key press, store the value of the key in Vx.

// All execution stops until a key is pressed, then the value of that key is stored in Vx.

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
    #[test]
    fn test_get_value_of_key() {
        let mut my_chip = build_chip8();
        my_chip.keyboard[15] = true; 

        assert_eq!(get_value_of_key(15), 0xF);
        assert_eq!(get_value_of_key(14), 0xB);
        assert_eq!(get_value_of_key(13), 0x0);
        assert_eq!(get_value_of_key(12), 0xA);
        assert_eq!(get_value_of_key(11), 0xE);
        assert_eq!(get_value_of_key(10), 0x9);
        assert_eq!(get_value_of_key(9), 0x8);
        assert_eq!(get_value_of_key(8), 0x7);
        assert_eq!(get_value_of_key(7), 0xD);
        assert_eq!(get_value_of_key(6), 0x6);
        assert_eq!(get_value_of_key(5), 0x5);
        assert_eq!(get_value_of_key(4), 0x4);
        assert_eq!(get_value_of_key(3), 0xC);
        assert_eq!(get_value_of_key(2), 0x3);
        assert_eq!(get_value_of_key(1), 0x2);
        assert_eq!(get_value_of_key(0), 0x1);
 
        
    }
    #[test]
    fn test_fx0a_test_cpu_halts() {
        let mut my_chip = build_chip8();
        my_chip.pc=514;
        my_chip.delay_timer.val = 10;
        my_chip.sound_timer.val = 10;
        my_chip.execute("FA0A".to_string());

        assert_eq!(my_chip.pc, 512);
        assert_eq!(my_chip.delay_timer.val, 11);
        assert_eq!(my_chip.sound_timer.val, 11);
        
    }
    #[test]
    fn test_fx0a() {
        let mut my_chip = build_chip8();
        my_chip.keyboard[15] = true;
        
        my_chip.execute("FA0A".to_string());
        
        assert_eq!(my_chip.regs[0xA], 0xF);
        
        my_chip.keyboard[12] = true;
        my_chip.execute("FA0A".to_string());
        assert_eq!(my_chip.regs[0xA], 0xA); 
        
        my_chip.keyboard[3] = true;
        
        
        
        my_chip.execute("FA0A".to_string());
        
        assert_eq!(my_chip.regs[0xA], 0xC);
        assert_eq!(my_chip.pc, 512);
        assert_eq!(my_chip.delay_timer.val, 0);
        assert_eq!(my_chip.sound_timer.val, 0);
        
    }
    
// Fx07 - LD Vx, DT
// Set Vx = delay timer value.


    #[test]
    fn test_fx07() {
        let mut my_chip = build_chip8();
        my_chip.delay_timer.val = 50;
        my_chip.execute("FA07".to_string()); 
        assert_eq!(my_chip.regs[0xA], 50); 
        my_chip.delay_timer.tick(); 
        my_chip.execute("FA07".to_string()); 
        assert_eq!(my_chip.regs[0xA], 49);
    }


    #[test]
    fn test_sound_timer() {
        let mut my_chip = build_chip8();
        my_chip.sound_timer.val = 50;
        my_chip.sound_timer.tick(); 
        assert_eq!(my_chip.sound_timer.val, 49);

    }
    #[test]
    fn test_sound_timer_doesnt_go_below_zero() {
        let mut my_chip = build_chip8();
        my_chip.sound_timer.val = 0;
        my_chip.sound_timer.tick(); 
        assert_eq!(my_chip.sound_timer.val, 0);
    } 
    #[test]
    fn test_delay_timer_doesnt_go_below_zero() {
        let mut my_chip = build_chip8();
        my_chip.delay_timer.val = 0;
        my_chip.delay_timer.tick(); 
        assert_eq!(my_chip.delay_timer.val, 0);
    }
    #[test]
    fn test_delay_timer() {
        let mut my_chip = build_chip8();
        my_chip.delay_timer.val = 60;
        my_chip.delay_timer.tick(); 
        assert_eq!(my_chip.delay_timer.val, 59);
    }
    /*
    
    Fx15 - LD DT, Vx
    Set delay timer = Vx.
    */
    #[test]
    fn test_fx18() {
        let mut my_chip = build_chip8();
        my_chip.sound_timer.val = 10 ;
        my_chip.regs[0xA] = 240;
        my_chip.execute("FA18".to_string() );
        assert_eq!(my_chip.sound_timer.val, my_chip.regs[0xA] as u32);


    }
    #[test]
    fn test_fx15() {
        let mut my_chip = build_chip8();
        my_chip.delay_timer.val = 10 ;
        my_chip.regs[0xA] = 240;
        my_chip.execute("FA15".to_string() );
        assert_eq!(my_chip.delay_timer.val, my_chip.regs[0xA] as u32);
    }
    #[test]
    fn test_ex9e_7() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 0xA;
        my_chip.keyboard[12] = false;
        my_chip.execute("EC9E".to_string()); 
        assert_eq!(my_chip.pc, 512 ); 
    }
    #[test]
    fn test_ex9e_6() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 0xA;
        my_chip.keyboard[12] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[12] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    #[test]
    fn test_ex9e_5() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 3;
        my_chip.keyboard[2] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[2] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    #[test]
    fn test_ex9e_4() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 2;
        my_chip.keyboard[1] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[1] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    #[test]
    fn test_ex9e_3() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 2;
        my_chip.keyboard[1] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[1] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    #[test]
    fn test_ex9e_2() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 1;
        my_chip.keyboard[0] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[0] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    // DXYN (display/draw)
    #[test]
    fn test_ex9e() {
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 12;
        my_chip.keyboard[3] = true;
        my_chip.execute("EC9E".to_string());
        
        assert_eq!(my_chip.pc, 514 );
        my_chip.keyboard[3] = false;
        my_chip.execute("EC9E".to_string());
        assert_eq!(my_chip.pc, 514 );
    }
    #[test]
    fn test_make_into_8_field() { 
        assert_eq!("00000001", make_into_8_field("1".to_string()));
        assert_eq!("00111110", make_into_8_field( "111110".to_string() ));
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


// ExA1 - SKNP Vx
// Skip next instruction if key with the value of Vx is not pressed.

// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.


    #[test]
    fn test_exa1(){
        let mut my_chip = build_chip8();
        assert_eq!(my_chip.pc, 512 );
        my_chip.regs[0xC] = 12;
        my_chip.keyboard[3] = true;
        my_chip.execute("ECA1".to_string());
        
        assert_eq!(my_chip.pc, 512 );
        my_chip.keyboard[3] = false;
        my_chip.execute("ECA1".to_string());
        assert_eq!(my_chip.pc, 514 );

    }
    #[test]
    fn test_get_reg_value_from_hex(){
        let mut my_chip = Chip8::new();

        my_chip.regs[ 0xA ] = 3;
        my_chip.regs[ 0x0 ] = 2;
        my_chip.regs[ 0x1 ] = 1;
        my_chip.regs[ 0x2 ] = 23;
        my_chip.regs[ 0x3 ] = 12;
        my_chip.regs[ 0x4 ] = 13;
        my_chip.regs[ 0xC ] = 4;
        my_chip.regs[ 0xF ] = 5;

        assert_eq!(my_chip.get_reg_value_from_hex('A'), 3);
        assert_eq!(my_chip.get_reg_value_from_hex('2'), 23);
        assert_eq!(my_chip.get_reg_value_from_hex('3'), 12);
        assert_eq!(my_chip.get_reg_value_from_hex('4'), 13);
        assert_eq!(my_chip.get_reg_value_from_hex('C'), 4);

    }
    #[test]
    fn test_convert_vx_value_into_keyboard_index_value(){

        assert_eq!(convert_vx_value_into_keyboard_index_value('1'.to_string()), 0);
        assert_eq!(convert_vx_value_into_keyboard_index_value('2'.to_string()), 1);
        assert_eq!(convert_vx_value_into_keyboard_index_value('3'.to_string()), 2);
        assert_eq!(convert_vx_value_into_keyboard_index_value('C'.to_string()), 3);
        assert_eq!(convert_vx_value_into_keyboard_index_value('4'.to_string()), 4);
        assert_eq!(convert_vx_value_into_keyboard_index_value('5'.to_string()), 5);
        assert_eq!(convert_vx_value_into_keyboard_index_value('6'.to_string()), 6);
        assert_eq!(convert_vx_value_into_keyboard_index_value('D'.to_string()), 7);
        assert_eq!(convert_vx_value_into_keyboard_index_value('7'.to_string()), 8);
        assert_eq!(convert_vx_value_into_keyboard_index_value('8'.to_string()), 9);
        assert_eq!(convert_vx_value_into_keyboard_index_value('9'.to_string()), 10);
        assert_eq!(convert_vx_value_into_keyboard_index_value('E'.to_string()), 11);
        assert_eq!(convert_vx_value_into_keyboard_index_value('A'.to_string()), 12);
        assert_eq!(convert_vx_value_into_keyboard_index_value('0'.to_string()), 13);
        assert_eq!(convert_vx_value_into_keyboard_index_value('B'.to_string()), 14);
        assert_eq!(convert_vx_value_into_keyboard_index_value('F'.to_string()), 15);
    }
    #[test]
    fn test_dxyn_4(){

        let mut my_chip = Chip8::new();
        my_chip.vram[0][0] = '1';
        
        my_chip.load(&vec![0b11111111,0b11111111]);
        my_chip.execute("A200".to_string());
        my_chip.execute("D001".to_string());
        
        assert_eq!(my_chip.vram[0][0], '0');
        assert_eq!(my_chip.vram[0][1], '1');
        assert_eq!(my_chip.vram[0][2], '1');
        assert_eq!(my_chip.vram[0][3], '1');
        assert_eq!(my_chip.vram[0][4], '1');
        assert_eq!(my_chip.vram[0][5], '1');
        assert_eq!(my_chip.vram[0][6], '1');
        assert_eq!(my_chip.vram[0][7], '1');
        
        my_chip.execute("D001".to_string()); 
        assert_eq!(my_chip.regs[0xf], 1);
        
        assert_eq!(my_chip.vram[0][0], '1');
        assert_eq!(my_chip.vram[0][1], '0');
        assert_eq!(my_chip.vram[0][2], '0');
        assert_eq!(my_chip.vram[0][3], '0');
        assert_eq!(my_chip.vram[0][4], '0');
        assert_eq!(my_chip.vram[0][5], '0');
        assert_eq!(my_chip.vram[0][6], '0');
        assert_eq!(my_chip.vram[0][7], '0');
    }
    #[test]
    fn test_dxyn_carry(){

        let mut my_chip = Chip8::new();
        my_chip.vram[0][0] = '1';
        
        my_chip.load(&vec![0b11111111,0b11111111]);
        my_chip.execute("A200".to_string());
        my_chip.execute("D001".to_string());
        
        assert_eq!(my_chip.regs[0xf], 1);
        
        my_chip.execute("D001".to_string()); 
        assert_eq!(my_chip.regs[0xf], 1);

        my_chip.vram[0][0] = '0';
        my_chip.execute("D001".to_string()); 
        assert_eq!(my_chip.regs[0xf], 0);
    }
    #[test]
    fn test_dxyn_wrap_around(){

        let mut my_chip = Chip8::new();
        my_chip.load(&vec![0b11111111,0b11111111]);
        // x
        my_chip.regs[0] = 61;
        
        // y
        my_chip.regs[1] = 31;
         // 31 - 0 y inc
         // 60 - 3 x inc
        my_chip.execute("A200".to_string());
        my_chip.execute("D012".to_string());



        
        // y x 
        assert_eq!( my_chip.vram[31][60], '0' );
        assert_eq!( my_chip.vram[31][61], '1' );
        assert_eq!( my_chip.vram[31][62], '1' );
        assert_eq!( my_chip.vram[31][63], '1' ); 
        assert_eq!( my_chip.vram[31][0], '1' );
        assert_eq!( my_chip.vram[31][1], '1' );
        assert_eq!( my_chip.vram[31][2], '1' );
        assert_eq!( my_chip.vram[31][3], '1' );
        assert_eq!( my_chip.vram[31][4], '1' );
        assert_eq!( my_chip.vram[31][5], '0' );
        assert_eq!( my_chip.vram[31][6], '0' ); 


        assert_eq!( my_chip.vram[0][61], '1' );
        assert_eq!( my_chip.vram[0][62], '1' );
        assert_eq!( my_chip.vram[0][63], '1' ); 
        assert_eq!( my_chip.vram[0][0], '1' );
        assert_eq!( my_chip.vram[0][1], '1' );
        assert_eq!( my_chip.vram[0][2], '1' );
        assert_eq!( my_chip.vram[0][3], '1' );
        assert_eq!( my_chip.vram[0][4], '1' );
        assert_eq!( my_chip.vram[0][5], '0' );
        assert_eq!( my_chip.vram[0][6], '0' ); 
    }
    #[test]
    fn test_dxyn3(){

        let mut my_chip = Chip8::new();
        my_chip.load(&vec![0b11100000,0b0]);
        my_chip.execute("A200".to_string());
        my_chip.execute("D001".to_string());

        
        
        assert_eq!( my_chip.vram[0][0], '1' );
        assert_eq!( my_chip.vram[0][1], '1' );
        assert_eq!( my_chip.vram[0][2], '1' );
        assert_eq!( my_chip.vram[0][3], '0' );
        assert_eq!( my_chip.vram[0][4], '0' );
        assert_eq!( my_chip.vram[0][5], '0' );
        assert_eq!( my_chip.vram[0][6], '0' );
        assert_eq!( my_chip.vram[0][7], '0' );
    }
    #[test]
    fn test_dxyn2(){

        let mut my_chip = Chip8::new();
        my_chip.load(&vec![0b00000111,0b0]);
        my_chip.execute("A200".to_string());
        my_chip.execute("D001".to_string());

        
        
        assert_eq!( my_chip.vram[0][0], '0' );
        assert_eq!( my_chip.vram[0][1], '0' );
        assert_eq!( my_chip.vram[0][2], '0' );
        assert_eq!( my_chip.vram[0][3], '0' );
        assert_eq!( my_chip.vram[0][4], '0' );
        assert_eq!( my_chip.vram[0][5], '1' );
        assert_eq!( my_chip.vram[0][6], '1' );
        assert_eq!( my_chip.vram[0][7], '1' );
    }
    #[test]
    fn test_dxyn(){
        
        
        // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
        
        
        let mut my_chip = Chip8::new();
        
        my_chip.load(&vec![0b11111111,0b11111111]);
        
        my_chip.vram[0][0] = '1';
        my_chip.vram[0][2] = '1';
        my_chip.vram[0][4] = '1';
        my_chip.vram[0][6] = '1';
        my_chip.execute("A200".to_string());
        my_chip.execute("D001".to_string());
        
        assert_eq!( my_chip.vram[0][0], '0' );
        assert_eq!( my_chip.vram[0][1], '1' );
        assert_eq!( my_chip.vram[0][2], '0' );
        assert_eq!( my_chip.vram[0][3], '1' );
        assert_eq!( my_chip.vram[0][4], '0' );
        assert_eq!( my_chip.vram[0][5], '1' );
        assert_eq!( my_chip.vram[0][6], '0' );
        assert_eq!( my_chip.vram[0][7], '1' );
        
        let mut my_chip2 = Chip8::new();
        my_chip2.load(&vec![0b11111111,0b0]);
        my_chip2.execute("A200".to_string());
        my_chip2.execute("D001".to_string());

        
        
        assert_eq!( my_chip2.vram[0][0], '1' );
        assert_eq!( my_chip2.vram[0][1], '1' );
        assert_eq!( my_chip2.vram[0][2], '1' );
        assert_eq!( my_chip2.vram[0][3], '1' );
        assert_eq!( my_chip2.vram[0][4], '1' );
        assert_eq!( my_chip2.vram[0][5], '1' );
        assert_eq!( my_chip2.vram[0][6], '1' );
        assert_eq!( my_chip2.vram[0][7], '1' );
        
        
        
        
        
        
    }
    #[test]
    fn test_make_into_n_field(){

        assert_eq!(make_into_n_field(4, "E0".to_string()), "00E0");
        assert_eq!(make_into_n_field(8, "E0".to_string()), "000000E0");
    }
    

    // 00EE - RET
    // Return from a subroutine.
    
    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    
#[test]
fn test_00ee(){ 

    let mut my_chip = build_chip8(); 
    my_chip.stack.push(515);
    my_chip.stack.push(529);

    my_chip.execute("00EE".to_string());

    assert_eq!(my_chip.pc, 529);
    assert_eq!(my_chip.stack.len(), 1);
    assert_eq!(my_chip.stack[0], 515);


} 
    // 8xyE - SHL Vx {, Vy}
    // Set Vx = Vx SHL 1.
    
    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    
#[test]
fn test_8xye_3(){ 


    let mut my_chip = build_chip8(); 
    my_chip.regs[0xA] = 0b10000101;  
    my_chip.regs[0xF] = 0; 

    my_chip.execute("8A0E".to_string());
    
    assert_eq!(my_chip.regs[0xF], 1);
    assert_eq!(my_chip.regs[0xA], 0b00001010);

}
#[test]
fn test_8xye_2(){ 


    let mut my_chip = build_chip8(); 
    my_chip.regs[0xA] = 0b00000100; 
    let x = 0b00000100; 
    my_chip.regs[0xF] = 1; 

    my_chip.execute("8A0E".to_string());
    
    assert_eq!(my_chip.regs[0xF], 0);
    assert_eq!(my_chip.regs[0xA], x*2);

}
#[test]
fn test_8xye(){ 


    let mut my_chip = build_chip8(); 
    my_chip.regs[0xA] = 0b00000101; 
    my_chip.regs[0xF] = 0; 

    my_chip.execute("8A0E".to_string());
    
    assert_eq!(my_chip.regs[0xF], 1);
    assert_eq!(my_chip.regs[0xA], 0b00001010);

}
    /*
    
8xy7 - SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.

If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

*/
#[test]
fn test_8xy7_2(){ 
    let mut my_chip = build_chip8(); 
    my_chip.regs[0x1] = 0b11100100; 
    my_chip.regs[0x0] = 0b11110100; 
    let x = my_chip.regs[0x0];  
    let y = my_chip.regs[0x1];  
    let res = y.wrapping_sub(x);
    my_chip.execute("8017".to_string()); 
    assert_eq!(my_chip.regs[0x0], res);
    assert_eq!(my_chip.regs[0xf], 0);  
}
    #[test]
    fn test_8xy7(){ 
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b11100100; 
        my_chip.regs[0x1] = 0b11110100; 
        let x = my_chip.regs[0x0];  
        let y = my_chip.regs[0x1];  
        let res = y.wrapping_sub(x);
        my_chip.execute("8017".to_string()); 
        assert_eq!(my_chip.regs[0x0], res);
        assert_eq!(my_chip.regs[0xf], 1);  
    }
    #[test]
    fn test_8xy6_2(){ 
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b11100100; 
        let x = my_chip.regs[0x0];  
        my_chip.execute("8016".to_string()); 
        assert_eq!(my_chip.regs[0x0], x/2 as u8);
        assert_eq!(my_chip.regs[0xf], 0);  
    }
    #[test]
    fn test_8xy6(){ 
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b11100101; 
        let x = my_chip.regs[0x0];  
        my_chip.execute("8016".to_string()); 
        assert_eq!(my_chip.regs[0x0], x/2 as u8);
        assert_eq!(my_chip.regs[0xf], 1);  
    }
    #[test]
    fn test_8xy5_pt2(){
        // vx xor vy
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b11100101;
        my_chip.regs[0x1] = 0b01111001;
        let x = my_chip.regs[0x0];
        let y = my_chip.regs[0x1]; 
        let sn = x - y;
        my_chip.regs[0xf] = 0;
        my_chip.execute("8015".to_string());

        assert_eq!(my_chip.regs[0x0], sn as u8);
        assert_eq!(my_chip.regs[0xf], 1);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    
    }
    #[test]
    fn test_8xy5(){
        // vx xor vy
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b01100101;
        my_chip.regs[0x1] = 0b01111001;
        let x = my_chip.regs[0x0];
        let y = my_chip.regs[0x1];
        let sn = x.wrapping_sub(y);
        my_chip.regs[0xf] = 1;
        my_chip.execute("8015".to_string());

        assert_eq!(my_chip.regs[0x0], sn as u8);
        assert_eq!(my_chip.regs[0xf], 0);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    
    }
    #[test]
    fn test_8xy4_pt2(){
        // vx xor vy
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b01100101;
        my_chip.regs[0x1] = 0b01111001;
        let x = my_chip.regs[0x0];
        let y = my_chip.regs[0x1];
        let sn = x as u16+y as u16;
        my_chip.regs[0xf] = 1;
        my_chip.execute("8014".to_string());

        assert_eq!(my_chip.regs[0x0], sn as u8);
        assert_eq!(my_chip.regs[0xf], 0);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    
    }
    #[test]
    fn test_8xy4(){
        // vx xor vy
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b11100101;
        my_chip.regs[0x1] = 0b01111001;
        let x = my_chip.regs[0x0];
        let y = my_chip.regs[0x1];
        let sn = x as u16+y as u16;
        my_chip.execute("8014".to_string());

        assert_eq!(my_chip.regs[0x0], sn as u8);
        assert_eq!(my_chip.regs[0xf], 1);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    
    }
    #[test]
    fn test_8xy3(){
        // vx xor vy
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b01100101;
        my_chip.regs[0x1] = 0b01111001;
        my_chip.execute("8013".to_string());

        assert_eq!(my_chip.regs[0x0], 0b00011100);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    
    }
    #[test]
    fn test_8xy2(){

        // 8xy1 - AND Vx, Vy
        // Set Vx = Vx AND Vy.
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b01100101;
        my_chip.regs[0x1] = 0b01111001;
        my_chip.execute("8012".to_string());

        assert_eq!(my_chip.regs[0x0], 0b01100001);
        assert_eq!(my_chip.regs[0x1], 0b01111001);
    }
    #[test]
    fn test_8xy1(){


        // 8xy1 - OR Vx, Vy
        // Set Vx = Vx OR Vy.
        let mut my_chip = build_chip8(); 
        my_chip.regs[0x0] = 0b01100101;
        my_chip.regs[0x1] = 0b01111001;
        my_chip.execute("8011".to_string());

        assert_eq!(my_chip.regs[0x0], 0b01111101);
        assert_eq!(my_chip.regs[0x1], 0b01111001);

        

    }
    #[test]
    fn test_a_nnn(){
        let mut my_chip = build_chip8(); 
        my_chip.execute("A111".to_string());
        assert_eq!(my_chip.index_register, 0x111);
        
        
    }
    #[test]
    fn test_7_xnn(){
        let mut my_chip = build_chip8(); 
        my_chip.execute("6F08".to_string());
        assert_eq!(my_chip.regs[0xf],8);

        my_chip.execute("7F08".to_string());
        assert_eq!(my_chip.regs[0xf],16);

        my_chip.execute("7F08".to_string());
        assert_eq!(my_chip.regs[0xf],24);

        my_chip.execute("7F18".to_string());
        assert_eq!(my_chip.regs[0xf],48);

        my_chip.execute("7018".to_string());
        assert_eq!(my_chip.regs[0x0],24);
        
    }

    /*
    
5xy0 - SE Vx, Vy
Skip next instruction if Vx = Vy.

The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.

*/
    #[test]
    fn test_5_xy0(){
        let mut my_chip = build_chip8(); 
        my_chip.regs[0xA] = 20;
        my_chip.regs[0xB] = 21;
        my_chip.execute("5AB0".to_string());
        assert_eq!(my_chip.pc, 512);
        my_chip.regs[0xB] = 20;
        my_chip.execute("5AB0".to_string());
        assert_eq!(my_chip.pc, 514);
        
    }

// Cxkk - RND Vx, byte
// Set Vx = random byte AND kk.

// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.

// can't test randomness, but we can know 0 and N = 0.

#[test]
fn test_cxnn_2(){ 
    let mut my_chip = build_chip8(); 
    my_chip.regs[0xA] = 132;
    while  my_chip.regs[0xA] != 0 {
        my_chip.execute("CA49".to_string());  
    }
    // because there's a 1/255 chance of reg 

    assert_eq!(my_chip.regs[0xA], 0); 
}
    #[test]
    fn test_cxnn(){ 
        let mut my_chip = build_chip8(); 
        my_chip.regs[0xA] = 132;
        my_chip.execute("CA00".to_string());
        // because there's a 1/255 chance of reg 

        assert_eq!(my_chip.regs[0xA], 0); 
    }
    // 9xy0 - SNE Vx, Vy
    // Skip next instruction if Vx != Vy.
    
    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    #[test]
    fn test_9xy0(){
        let mut my_chip = build_chip8(); 
        my_chip.regs[0xA] = 20;
        my_chip.regs[0xB] = 20;
        my_chip.execute("9AB0".to_string());
        
        assert_eq!(my_chip.pc, 512);
        my_chip.regs[0xB] = 21;
        my_chip.execute("9AB0".to_string());
        assert_eq!(my_chip.pc, 514);

    }
    #[test]
    fn test_6_xnn(){
        let mut my_chip = build_chip8(); 
        my_chip.execute("6F08".to_string());
        assert_eq!(my_chip.regs[0xf],8);
        
    }
    #[test]
    fn test_1_nnn(){
        let mut my_chip = build_chip8(); 
        my_chip.execute("1208".to_string());
        assert_eq!(my_chip.pc, 0x208);
    }
    #[test]
    fn test_fetch(){
        let mut my_chip = build_chip8();  
        assert_eq!(my_chip.pc, 512);
        let inst = my_chip.fetch();
        assert_eq!(my_chip.pc, 514);
        assert_eq!( (2,3), inst  );
        let inst2 = my_chip.fetch();
        assert_eq!(my_chip.pc, 516);
        assert_eq!( (5,7), inst2 );  
    }

    /*
    
    2nnn - CALL addr
    Call subroutine at nnn.

    The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.

    */
    #[test]
    fn test_2nnn(){
        let mut my_chip = build_chip8();  
        my_chip.execute("2555".to_string());

        assert_eq!(my_chip.stack.len(), 1);
        assert_eq!(my_chip.stack[0], 512);
        assert_eq!(my_chip.pc, 1365);

    }
    #[test]
    fn test_decode(){
        let mut my_chip = build_chip8();  
        assert_eq!(my_chip.pc, 512);
        let inst = my_chip.fetch();
        assert_eq!(my_chip.pc, 514);
        assert_eq!( (2,3), inst  );
        let decoded = my_chip.decode(inst);
        assert_eq!(decoded, "203");
        let inst2 = my_chip.fetch();
        let decoded2 = my_chip.decode(inst2);
        assert_eq!(my_chip.pc, 516);
        assert_eq!( (5,7), inst2 );  
        assert_eq!(decoded2, "507");
    }
    #[test]
    fn test_00_e0() {
        let mut my_chip = build_chip8(); 
         my_chip.vram[0][0] = '1';
         my_chip.vram[31][40] = '1';
         my_chip.vram[20][30] = '1';
         my_chip.vram[15][50] = '1';

         my_chip.execute("E0".to_string());

         for (i, &row) in my_chip.vram.iter().enumerate() {
             for(j,&bit) in row.iter().enumerate(){
                 assert_eq!(bit, '0');
             }
         } 
    }

    /*
4xkk - SNE Vx, byte
Skip next instruction if Vx != kk.

The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
*/
#[test]
fn test_4xkk() {  
   let mut my_chip = build_chip8(); 
   my_chip.regs[0xb] = 32; 
   
   my_chip.execute("4B20".to_string());
   assert_eq!(my_chip.pc, 512);
 
   my_chip.execute("4BC0".to_string());
   assert_eq!(my_chip.pc, 514)

}




/* 
5xy0 - SE Vx, Vy
Skip next instruction if Vx = Vy.

The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
 */
 #[test]
 fn test_5xy0() {  
    let mut my_chip = build_chip8(); 
    my_chip.regs[0xb] = 32;
    my_chip.regs[0xc] = 32;
    
    my_chip.execute("5BC0".to_string());
    assert_eq!(my_chip.pc, 514);
    
    my_chip.regs[0xc] = 33;
    my_chip.execute("5BC0".to_string());
    assert_eq!(my_chip.pc, 514)
    
}


#[test]
fn test_load() {  
    let mut vec = Vec::new();
    vec.push(1);  
    vec.push(2);
    vec.push(234);
    vec.push(14);
    
    let mut my_chip = chip8::Chip8::new();
    
    my_chip.load(&vec);
    
    assert_eq!(my_chip.ram[0x200],1);
    assert_eq!(my_chip.ram[0x201],2);
    assert_eq!(my_chip.ram[0x202],234);
    assert_eq!(my_chip.ram[0x203],14);  
} 

/*

Fx1E - ADD I, Vx
Set I = I + Vx.

The values of I and Vx are added, and the results are stored in I.

*/
#[test]
fn test_fx1e(){
    let mut my_chip = build_chip8(); 
    my_chip.index_register = 10;
    my_chip.regs[0xA] = 10;
    my_chip.execute("FA1E".to_string());

    assert_eq!(my_chip.index_register, 20)
    
}
/* 
Fx29 - LD F, Vx
Set I = location of sprite for digit Vx.

The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
*/


    #[test]
    fn test_fx29(){
        let mut my_chip = build_chip8(); 
        my_chip.index_register = 0;
        my_chip.regs[0xA] = 0xA;
        my_chip.execute("FA29".to_string());

        assert_eq!(my_chip.index_register, my_chip.font_locations[0xA] as u16);
        
    }

/* 

Fx33 - LD B, Vx
Store BCD representation of Vx in memory locations I, I+1, and I+2.

For example, if VX contains 156 (or 9C in hexadecimal), it would put the number 1 at the address in I, 5 in address I + 1, and 6 in address I + 2.
*/
    #[test]
    fn test_fx33(){
        let mut my_chip = build_chip8(); 
        my_chip.index_register = 520;
        my_chip.regs[0xA] = 156;
        my_chip.execute("FA33".to_string());
        
        assert_eq!(my_chip.ram[520], 1);
        assert_eq!(my_chip.ram[521], 5);
        assert_eq!(my_chip.ram[522], 6);

    }

/* 
Fx55 - LD [I], Vx
Store registers V0 through Vx in memory starting at location I.

The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.

*/ 

#[test]
fn test_fx55(){
    let mut my_chip = build_chip8(); 
    my_chip.index_register = 520;
    my_chip.regs[0x1] = 12;
    my_chip.regs[0x5] = 13;
    my_chip.regs[0x7] = 156;
    my_chip.regs[0x8] = 156;
    my_chip.regs[0x9] = 156;
    my_chip.regs[0xA] = 156;
    my_chip.regs[0xB] = 56;
    my_chip.regs[0xC] = 15;
    my_chip.regs[0xD] = 16;
    my_chip.execute("FA55".to_string());
    
    assert_eq!(my_chip.ram[520], 0);
    assert_eq!(my_chip.ram[521], 12);
    assert_eq!(my_chip.ram[my_chip.index_register as usize + 0xA], 156);
    assert_eq!(my_chip.ram[my_chip.index_register as usize + 0xB], 0);
    assert_eq!(my_chip.ram[my_chip.index_register as usize + 0xD], 0);

}

/* 

Fx65 - LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.

    */

    #[test] // 2 3 5 7
    fn test_fx65(){
        let mut my_chip = build_chip8(); 
        my_chip.index_register = 512; 
        my_chip.ram[my_chip.index_register as usize + 0xA] = 251;
        my_chip.execute("FA65".to_string());
        
        assert_eq!(my_chip.regs[0], 2);
        assert_eq!(my_chip.regs[1], 3);
        assert_eq!(my_chip.regs[2], 5);
        assert_eq!(my_chip.regs[3], 7); 
        assert_eq!(my_chip.regs[0xA], 251); 
    
    }
}
