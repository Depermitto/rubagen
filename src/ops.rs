use bmp::{Image, consts::WHITE};
use crate::data::CODES;

pub trait BarcodeGen {
    /// Modifies \[`Self`\] with an encoded `input` using
    /// [Code 39](https://en.wikipedia.org/wiki/Code_39).
    ///
    /// Returns \[`Self`]
    ///
    /// # Example
    ///
    /// basic usage:
    /// ```
    /// let mut img = Image::new(width, height);
    /// img.code39_gen("*PINEAPPLE*");
    /// ```
    fn code39_gen(&mut self, input: &str) -> Self;
    // fn code128_gen(&mut self, input: &str, scale: T) -> Self;
}

impl BarcodeGen for Image {
    fn code39_gen(&mut self, input: &str) -> Self {
        // Image stats
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut white: bool = false;

        for ch in input.chars() {
            // This loop codes a single character [ch] from input
            // with a 1px wide white line after
            for thickness in CODES[&ch] {
                // Repeat [thickness] time
                for _ in 0..thickness {
                    // Paint vertical line white
                    while white && y < self.get_height() {
                        self.set_pixel(x, y, WHITE);
                        y += 1;
                    }
                    // Next line
                    y = 0;
                    x += 1;
                }
                // Switch colors
                white = !white;
            }
        }

        self.clone()
    }
}