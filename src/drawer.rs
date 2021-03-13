
extern crate sdl2;  
use sdl2::{EventPump, Sdl, pixels::Color, render::Canvas, video::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::{io, time::Duration}; 
use std::thread; 

pub struct Drawer{
    canvas: Canvas<Window>,  
}

impl Drawer{

    pub fn new(sdl_context: &Sdl) -> Drawer { 
        let video_subsystem = sdl_context.video().unwrap();
     
          
        // let  event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
            .unwrap();
     
        let canvas = window.into_canvas().build().unwrap(); 

        return Drawer{
            canvas,  
        }
        
    }

    pub fn clear(&mut self){ 
        println!("Clearing!"); 
        self.canvas.set_draw_color(Color::WHITE); 
        self.canvas.clear();
        self.canvas.present(); 
        self.canvas.clear();
 

        self.canvas.set_draw_color(Color::BLACK); 
           
        let mut i = 0;
        // 'running: loop {
        //     i = (i + 1) % 255; 
        //     for event in self.event_pump.poll_iter() {
        //         match event {
        //             Event::Quit {..} |
        //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        //                 break 'running
        //             },
        //             _ => {}
        //         }
        //     }  
            
        //     if i == 100{
        //         break;
        //     } 
        //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // }
    } 
 
 
    pub fn draw(&mut self, sprite: [[char; 64]; 32]){
        
        self.canvas.clear();
        for i in 0..sprite.len(){ 
            for j in 0..sprite[i].len(){ 
                if sprite[i][j] == '1' {
                    self.canvas.set_draw_color(Color::BLACK);
                    self.canvas.fill_rect(Rect::new((j *10)as i32, (i*10) as i32, 10, 10)).ok();
                } 
                if sprite[i][j] == '0' {
                    self.canvas.set_draw_color(Color::WHITE);
                    self.canvas.fill_rect(Rect::new((j*10) as i32, (i*10) as i32, 10, 10)).ok(); 
                }
                
            }
        }

        self.canvas.present(); 
        
        let mut i = 0;
        // 'running: loop {
        //     i = (i + 1) % 255; 
        //     for event in self.event_pump.poll_iter() {
        //         match event {
        //             Event::Quit {..} |
        //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        //                 break 'running
        //             },
        //             _ => {}
        //         }
        //     }  
            
        //     if i == 1{
        //         break;
        //     } 
        //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // }
 
  
    }
} 


