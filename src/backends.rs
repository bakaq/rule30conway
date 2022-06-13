use std::sync::{
    Arc,
    Mutex,
};

use sdl2::{
    pixels::Color,
    event::Event,
    render::Canvas,
    video::Window,
    keyboard::Keycode,
};

use crate::simulation::Rule30Conway;

pub trait Backend {
    fn main_loop(&mut self);
}

pub struct SdlBackend {
    width: u32,
    height: u32,
    scale: u32,
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
}

impl SdlBackend {
    pub fn new(width: u32, height: u32, scale: u32) -> Self {
        // Setup SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Setup window
        let window = video_subsystem.window("rule30conway", width*scale, height*scale)
            .position_centered()
            .build()
            .unwrap();
        
        // Setup canvas
        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        SdlBackend { width, height, scale, sdl_context, canvas }
    }
}

impl Backend for SdlBackend {
    fn main_loop(&mut self) {
        // Setup Rule30Conway
        let rule30conway_mutex = Arc::new(Mutex::new(
                Rule30Conway::new(
                self.width as usize,
                self.height as usize,
            )
        ));

        {
            let rule30conway = rule30conway_mutex.lock().unwrap();
            rule30conway.draw_sdl(&mut self.canvas, self.scale as usize);
        }

        self.canvas.present();

        // Compute thread
        let speed_mutex = Arc::new(Mutex::new(1.0));
        let speed_mutex2 = Arc::clone(&speed_mutex);
        let rule30conway_mutex2 = Arc::clone(&rule30conway_mutex);
        std::thread::spawn(move || {
            loop {
                {
                    let mut rule30conway = rule30conway_mutex2.lock().unwrap();
                    rule30conway.step();
                }
                let speed = {
                    *speed_mutex2.lock().unwrap()
                };
                std::thread::sleep(std::time::Duration::from_secs_f64( (1.0/60.0) / speed ));
            }
        });

        // Main loop
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                // Handle events
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        *speed_mutex.lock().unwrap() *= 1.1;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        *speed_mutex.lock().unwrap() *= 0.9;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        *speed_mutex.lock().unwrap() *= 2.0;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                        *speed_mutex.lock().unwrap() *= 0.9;
                    }
                    Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                        *speed_mutex.lock().unwrap() = 1.0;
                    }
                    _ => {}
                }
            }

            // Clear canvas
            self.canvas.set_draw_color(Color::WHITE);
            self.canvas.clear();

            // Draw
            {
                let rule30conway = rule30conway_mutex.lock().unwrap();
                rule30conway.draw_sdl(&mut self.canvas, self.scale as usize);
            }

            self.canvas.present();

            // Delay between frames
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
