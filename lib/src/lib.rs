use std::io::{self, Write};

// i have no idea what this does but ok
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    p_data: Vec<Vec<Color>>,
    pub author: String,
    pub time: i64
}

pub mod utils {
    use super::*;
    pub fn reset() -> () {
        print!("\x1b[0m");
        io::stdout().flush().ok();
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let p = vec![vec![Color::White; width]; height];
        Self {
            width,
            height,
            p_data: p,
            author: "unknown".to_string(),
            time: 0
        }
    }
    pub fn set_author(&mut self, name: &str) -> () {
        self.author = name.to_string();
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, v: Color) -> () {
        if y-1 <= self.height-1 && x-1 <= self.width-1 {
            self.p_data[y-1][x-1] = v;
        }
    }
    pub fn set_time(&mut self, time: i64) -> () {
        self.time = time;
    }
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if y-1 <= self.height-1 && x-1 <= self.width-1 {
            return Some(self.p_data[y-1][x-1]);
        }
        None
    }
    pub fn format_pixel(&mut self, x: usize, y: usize) -> String {
        // help
        let as_usize = self.get_pixel(x, y).unwrap_or(Color::White) as usize;
        format!(
            "\x1b[{};{}m{}{}",
            as_usize + 30,
            as_usize + 40,
            as_usize,
            as_usize
        )
    }
}