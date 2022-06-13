use std::collections::VecDeque;

use sdl2::{
    pixels::Color,
    render::Canvas,
    video::Window,
    rect::Rect,
};

#[derive(Clone, Debug)]
struct Rule30 {
    buffer: VecDeque<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Rule30 {
    fn new(width: usize, height: usize) -> Self {
        let mut buffer = VecDeque::with_capacity(height+1);
        for _ in 0..height {
            buffer.push_back(vec![0; width]);
        }
        buffer[height-1][width/2] = 1;
        Rule30 { buffer, width, height }
    }

    fn step(&mut self) -> Vec<u8> {
        let new_line = self.get_next_line();
        self.buffer.push_back(new_line);
        self.buffer.pop_front().unwrap()
    }

    fn get_next_line(&self) -> Vec<u8> {
        let line = &self.buffer[self.height-1];
        let mut next_line = vec![0; self.width];

        for i in 0..self.width {
            let prev = line[if i == 0 { self.width-1 } else { i - 1 }];
            let mid = line[i];
            let next = line[(i+1) % self.width];
            let code = (prev << 2) | (mid << 1) | next;
            next_line[i] = (30 & (1 << code)) >> code;
        }

        next_line
    }

    fn draw_sdl(&self, canvas: &mut Canvas<Window>, offx: usize, offy: usize, scale: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.buffer[y][x];
                canvas.set_draw_color(if val == 0 { Color::WHITE } else { Color::BLACK });
                canvas.fill_rect(Some(
                    Rect::new(
                        ((x + offx) * scale) as i32,
                        ((y + offy) * scale) as i32, 
                        scale as u32, 
                        scale as u32,
                    )
                )).unwrap();
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Conway {
    using_a: bool,
    buffer_a: Vec<u8>,
    buffer_b: Vec<u8>,
    width: usize,
    height: usize,
}

impl Conway {
    fn new(width: usize, height: usize) -> Self {
        let buffer_a = vec![0; width*height];
        let buffer_b = vec![0; width*height];
        Self { using_a: true, buffer_a, buffer_b, width, height }
    }

    fn get_buffers(&self) -> (&Vec<u8>, &Vec<u8>) {
        if self.using_a {
            (&self.buffer_a, &self.buffer_b)
        } else {
            (&self.buffer_b, &self.buffer_a)
        }
    }

    fn get_buffers_mut(&mut self) -> (&mut Vec<u8>, &mut Vec<u8>) {
        if self.using_a {
            (&mut self.buffer_a, &mut self.buffer_b)
        } else {
            (&mut self.buffer_b, &mut self.buffer_a)
        }
    }

    fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let w = self.width;
                let h = self.height;
                let (b, swap_buffer) = self.get_buffers_mut();

                let i = y*w + x;
                let neighbors = match (x, y) {
                    (x,y) if x == 0 && y == 0 => {
                        b[i+1] + b[i+w] + b[i+w+1]
                    },
                    (x,y) if x == w-1 && y == 0 => {
                        b[i-1] + b[i+w-1] + b[i+w]
                    },
                    (x,y) if x == 0 && y == h-1 => {
                        b[i-w] + b[i-w+1] + b[i+1]
                    },
                    (x,y) if x == w-1 && y == h-1 => {
                        b[i-w-1] + b[i-w] + b[i-1]
                    },
                    (x,_) if x == 0 => {
                        b[i-w] + b[i-w+1] + b[i+1] + b[i+w] + b[i+w+1]
                    },
                    (x,_) if x == w-1 => {
                        b[i-w-1] + b[i-w] + b[i-1] + b[i+w-1] + b[i+w]
                    },
                    (_,y) if y == 0 => {
                        b[i-1] + b[i+1] + b[i+w-1] + b[i+w] + b[i+w+1]
                    },
                    (_,y) if y == h-1 => {
                        b[i-w-1] + b[i-w] + b[i-w+1] + b[i-1] + b[i+1]
                    },
                    _ => {
                        b[i-w-1] + b[i-w] + b[i-w+1]
                        + b[i-1] + b[i+1]
                        + b[i+w-1] + b[i+w] + b[i+w+1]
                    },
                };

                swap_buffer[i] = match (b[i], neighbors) {
                    (1,2) | (1,3) | (0,3) => 1,
                    _ => 0,
                };
            }

        }
        self.using_a = !self.using_a;
    }

    fn draw_sdl(&self, canvas: &mut Canvas<Window>, offx: usize, offy: usize, scale: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let (buffer, _) = self.get_buffers();
                let val = buffer[y*self.width + x];
                canvas.set_draw_color(if val == 0 { Color::WHITE } else { Color::BLACK });
                canvas.fill_rect(Some(
                    Rect::new(
                        ((x + offx) * scale) as i32,
                        ((y + offy) * scale) as i32, 
                        scale as u32, 
                        scale as u32,
                    )
                )).unwrap();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rule30Conway {
    rule30: Rule30,
    conway: Conway,
}

impl Rule30Conway {
    pub fn new(width: usize, height: usize) -> Self {
        let rule30 = Rule30::new(width, height/2);
        let conway = Conway::new(width, height/2);
        Rule30Conway { rule30, conway }
    }

    pub fn step(&mut self) {
        let line = self.rule30.step();
        let base_idx = (self.conway.height-1)*self.conway.width;
        let last_idx = base_idx + self.conway.width;
        let (buffer, _) = self.conway.get_buffers_mut();
        buffer[base_idx..last_idx]
            .copy_from_slice(&line);
        self.conway.step();
    }

    pub fn draw_sdl(&self, canvas: &mut Canvas<Window>, scale: usize) {
        self.rule30.draw_sdl(canvas, 0, self.conway.height, scale);
        self.conway.draw_sdl(canvas, 0, 0, scale);
    }
}

