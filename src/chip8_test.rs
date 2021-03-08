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
    
    // DXYN (display/draw)
    #[test]
    fn test_make_into_8_field() { 
        assert_eq!("00000001", make_into_8_field("1".to_string()));
        assert_eq!("00111110", make_into_8_field( "111110".to_string() ));
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

}
