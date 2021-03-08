
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
    sdl_context: Sdl,
    test: i16,
    event_pump: EventPump,
    history: Vec<(i32,i32, Vec<Vec<char>>)> 
}

impl Drawer{

    pub fn new() -> Drawer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
     
          
        let  event_pump = sdl_context.event_pump().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
            .unwrap();
     
        let canvas = window.into_canvas().build().unwrap(); 

        return Drawer{
            canvas,
            sdl_context,
            test: 32,
            event_pump, 
            history: Vec::new()
        }
        
    }

    pub fn clear(&mut self){ 
        println!("Clearing!");
        while !self.history.is_empty(){
            self.history.pop();
        }
        self.canvas.set_draw_color(Color::WHITE); 
        self.canvas.clear();
        self.canvas.present(); 
        self.canvas.clear();
 

        self.canvas.set_draw_color(Color::BLACK); 
           
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255; 
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }  
            
            if i == 100{
                break;
            } 
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    } 

    pub fn save_history(&mut self, x:i32,y:i32, sprite: Vec<Vec<char>>){
        self.history.push((x,y,sprite));

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
        
 
  
    }
}
pub fn fuck(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap(); 

    
     
    let x = 3;
    let y = 5;
    let p = Point::new(x, y);    
    let rect = Rect::new(50, 50, 50, 50); 
    let other = Rect::new(150, 150, 50, 50); 
    
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();
    
    
    canvas.clear();
    
    canvas.set_draw_color(Color::BLACK); 
    canvas.draw_rect(rect).ok();
    canvas.present();    
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    
    'running: loop {
        i = (i + 1) % 255; 
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }  
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
}


