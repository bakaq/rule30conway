#![allow(dead_code)]

use std::collections::VecDeque;

use sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

#[derive(Clone, Debug)]
struct Rule30 {
    buffer: VecDeque<Vec<u8>>,
    width: usize,
    height: usize,
}

fn rule30_next_line(line: &Vec<u8>) -> Vec<u8> {
    let width = line.len();
    let mut next_line = vec![0; width];

    for i in 0..width {
        let prev = line[if i == 0 { width-1 } else { i - 1 }];
        let mid = line[i];
        let next = line[(i+1) % width];
        let code = (prev << 2) | (mid << 1) | next;
        next_line[i] = (30 & (1 << code)) >> code;
    }

    next_line
}

impl Rule30 {
    fn new(width: usize, height: usize) -> Self {
        let mut buffer = VecDeque::new();
        for _ in 0..height {
            buffer.push_back(vec![0; width]);
        }
        buffer[height-1][width/2] = 1;
        Rule30 { buffer, width, height }
    }

    fn get_next_line(&mut self) -> Vec<u8> {
        let new_line = rule30_next_line(&self.buffer[self.height-1]);
        self.buffer.push_back(new_line);
        self.buffer.pop_front().unwrap()
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // Clear canvas
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Draw
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.buffer[y][x];
                canvas.set_draw_color(if val == 0 { Color::WHITE } else { Color::BLACK });
                canvas.fill_rect(Some(Rect::new(x as i32, y as i32, 1, 1))).unwrap();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Conway {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl Conway {
    fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width*height];
        Self { buffer, width, height }
    }

    fn step(&mut self) {
        todo!();
    }
}

#[derive(Clone, Debug)]
struct Rule30Conway {
    rule30: Rule30,
    conway: Conway,
}

impl Rule30Conway {
    // TODO
}


fn main() {
    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Setup window
    let window_size = [800, 600];
    let window = video_subsystem.window("rule30conway", window_size[0], window_size[1])
        .position_centered()
        .build()
        .unwrap();
    
    // Setup canvas
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    // Setup rule30
    let mut rule30 = Rule30::new(window_size[0] as usize, (window_size[1]/2) as usize);
    rule30.draw(&mut canvas);

    canvas.present();

    // Main loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            // Handle events
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                _ => {}
            }
        }

        // TODO: Calculate
        rule30.get_next_line();

        // Clear canvas
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // TODO: Draw
        rule30.draw(&mut canvas);

        canvas.present();

        // Delay between frames
        //std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
