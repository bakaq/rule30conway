use std::sync::{
    Arc,
    Mutex,
};

use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
};

use rule30conway::Rule30Conway;

fn main() {
    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Setup window
    let scale = 2;
    let screen_size = [500, 300];
    let window_size = [screen_size[0]*scale, screen_size[1]*scale];
    let window = video_subsystem.window("rule30conway", window_size[0], window_size[1])
        .position_centered()
        .build()
        .unwrap();
    
    // Setup canvas
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    // Setup Rule30Conway
    let rule30conway_mutex = Arc::new(Mutex::new(
            Rule30Conway::new(
            screen_size[0] as usize,
            screen_size[1] as usize,
        )
    ));

    {
        let rule30conway = rule30conway_mutex.lock().unwrap();
        rule30conway.draw(&mut canvas, scale as usize);
    }

    canvas.present();

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
    let mut event_pump = sdl_context.event_pump().unwrap();
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
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Draw
        {
            let rule30conway = rule30conway_mutex.lock().unwrap();
            rule30conway.draw(&mut canvas, scale as usize);
        }

        canvas.present();

        // Delay between frames
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
