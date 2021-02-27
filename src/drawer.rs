
extern crate sdl2;  
use sdl2::{Sdl, pixels::Color, render::Canvas, video::Window};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::{io, time::Duration}; 


pub struct Drawer{
    canvas: Canvas<Window>,
    sdl_context: Sdl
}

impl Drawer{

    pub fn new() -> Drawer {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
     
        let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
            .unwrap();
     
        let canvas = window.into_canvas().build().unwrap(); 

        return Drawer{
            canvas,
            sdl_context
        }
        
    }

    pub fn clear(&mut self){
        self.canvas.set_draw_color(Color::WHITE); 
        self.canvas.clear();
        self.canvas.present();
          
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            // canvas.set_draw_color(Color::RGB(i, 64, 255 - i)); 
            // canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            } 
            // canvas.draw_rect(rect).ok();
            
            // canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn draw(&mut self){


        let rect = Rect::new(50, 50, 50, 50); 
        let other = Rect::new(150, 150, 50, 50); 

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.canvas.present();

        self.canvas.clear();

        self.canvas.set_draw_color(Color::BLACK); 
        self.canvas.draw_rect(rect).ok();
        self.canvas.present();
          
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut i = 0;
        
        'running: loop {
            i = (i + 1) % 255;
            // canvas.set_draw_color(Color::RGB(i, 64, 255 - i)); 
            // canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            } 
            // canvas.draw_rect(rect).ok();
            
            // canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
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
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i)); 
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        } 
        // canvas.draw_rect(rect).ok();
        
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
 
}