extern crate sdl2;  
use std::{collections::HashSet, time::Duration};

use sdl2::{Sdl, event::Event, keyboard::{KeyboardUtil, Keycode}};
use sdl2::EventPump;
pub(crate) struct Keyboard{
   pub keyboard: KeyboardUtil,
   pub event_pump: EventPump,
   pub keys_pressed: [bool; 16],
   pub quit: bool
}

impl Keyboard{

    pub fn new(sdl_context:&Sdl) -> Keyboard { 

        let keyboard = sdl_context.keyboard();
        let event_pump = sdl_context.event_pump().unwrap();


        return Keyboard{  
            keyboard,
            event_pump,
            keys_pressed: [false;16],
            quit: false
        }

    }
    fn is_pressed(&mut self) -> bool{ 
        for i in 0..16{
            // println!("i am {}",self.keys_pressed[i]);
            if self.keys_pressed[i] == true {

                return true;
            };
        }
        false
    }
    fn reset_keys(&mut self){
        for i in 0..16{
            self.keys_pressed[i] = false;
        }
    }
    pub fn poll_keyboard(&mut self){ 

        // let prev_keys = std::collections::HashSet::new();
         

        
            
        self.reset_keys();
        self.event_pump.poll_event();
    
        let keys: HashSet<Keycode> = self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        // println!("HERE ARE KEYS {:?}", keys);

        // Get the difference between the new and old sets.
        // let new_keys = &keys - &prev_keys;
        // let old_keys = &prev_keys - &keys;
        if keys.contains(&Keycode::Num1){ 
            self.keys_pressed[0] = true;
        };
        if keys.contains(&Keycode::Num2){
            self.keys_pressed[1] = true;
        };
        if keys.contains(&Keycode::Num3){
            self.keys_pressed[2] = true;
        };
        if keys.contains(&Keycode::Num4){
            self.keys_pressed[3] = true;
        };
        if keys.contains(&Keycode::Q){
            self.keys_pressed[4] = true;
        };
        if keys.contains(&Keycode::W){
            self.keys_pressed[5] = true;
        };
        if keys.contains(&Keycode::E){
            self.keys_pressed[6] = true;
        };
        if keys.contains(&Keycode::R){
            self.keys_pressed[7] = true;
        };
        if keys.contains(&Keycode::A){
            self.keys_pressed[8] = true;
        };
        if keys.contains(&Keycode::S){
            self.keys_pressed[9] = true;
        };
        if keys.contains(&Keycode::D){
            self.keys_pressed[10] = true;
        };
        if keys.contains(&Keycode::F){
            self.keys_pressed[11] = true;
        };
        if keys.contains(&Keycode::Z){
            self.keys_pressed[12] = true;
        };
        if keys.contains(&Keycode::X){
            self.keys_pressed[13] = true;
        };
        if keys.contains(&Keycode::C){
            self.keys_pressed[14] = true;
        };
        if keys.contains(&Keycode::V){
            self.keys_pressed[15] = true;
        };


        if keys.contains(&Keycode::Num0){
            self.quit = true;
        };
 
  
    } 
}