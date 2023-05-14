use bmp::{Image, consts::WHITE};
use crate::data::CODES;

pub trait Code39Gen {
    fn code39_gen(&mut self, input: &str, scale: usize) -> Image;
}

impl Code39Gen for Image {
    fn code39_gen(&mut self, input: &str, scale: usize) -> Image {
        // Image stats
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut white: bool = false;

        for ch in input.chars() {
            // This loop codes a single character [ch] from input
            // with a 1px wide white line after
            for mut thickness in CODES[&ch] {
                // Repeat [thickness] times
                thickness *= scale;
                while thickness != 0 {
                    // Paint vertical line white
                    while white && y < self.get_height() {
                        self.set_pixel(x, y, WHITE);
                        y += 1;
                    }
                    // Next line
                    y = 0;
                    x += 1;
                    thickness -= 1;
                }
                // Switch colors
                white = !white;
            }
        }

        self.clone()
    }
}