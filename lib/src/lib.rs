use std::io::{self, Write};

// i have no idea what this does but ok
// nevermind i understand it a little
#[derive(Clone, Copy)]
/// color enum
/// 
/// used for almost all libdraw functions and goes from `0x0` (0,
/// Black) to `0x7` (7, White)
/// 
/// i'm planning to make it go from `0x0` to `0xf` (15)
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

/// contains information about a canvas
pub struct Canvas {
    /// canvas width
    pub width: usize,
    /// canvas height
    pub height: usize,
    /// canvas pixel data (shouldn't be accessible
    /// by a casual)
    p_data: Vec<Vec<Color>>,
    /// canvas author (you maybe)
    pub author: String,
    /// canvas time-of-creation
    pub time: i64
}

/// utilities (rust special)
pub mod utils {
    use super::*;
    /// resets the color mode for the terminal
    /// 
    /// basically a use-after-print function
    pub fn reset() -> () {
        print!("\x1b[0m");
        io::stdout().flush().ok();
    }
}

impl Canvas {
    /// makes a new canvas
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
    /// sets the author name to `name`
    pub fn set_author(&mut self, name: &str) -> () {
        self.author = name.to_string();
    }
    /// sets a pixel from `p_data` (pixel data) to any color from
    /// `Color` specified in `v` at position (`x`, `y`)
    pub fn set_pixel(&mut self, x: usize, y: usize, v: Color) -> () {
        if y-1 <= self.height-1 && x-1 <= self.width-1 {
            self.p_data[y-1][x-1] = v;
        }
    }
    /// sets the UNIX epoch time for the canvas in `time`
    pub fn set_time(&mut self, time: i64) -> () {
        self.time = time;
    }
    /// gets a pixel from `p_data` (pixel data) at position (`x`, `y`)
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if y-1 <= self.height-1 && x-1 <= self.width-1 {
            return Some(self.p_data[y-1][x-1]);
        }
        None
    }
    /// returns a formatted string containing a pixel at (`x`, `y`),
    /// colored with `Color` specified in the pixel
    /// 
    /// used to print a pixel, and use `utils::reset()` after you KNOW
    /// that you'll not use this anymore
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