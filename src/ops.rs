use bmp::{Image, consts::WHITE};
use crate::{
    error::BarcodeError,
    data::CODES39,
};

pub trait BarcodeGen {
    /// Modifies `Self` with an encoded `input` using
    /// [Code 39](https://en.wikipedia.org/wiki/Code_39).
    ///
    /// Returns Ok(`Self`) if the operation was a success,
    /// otherwise Err(`BarcodeError`)
    ///
    /// # Example
    ///
    /// basic usage:
    /// ```
    /// let mut img = Image::new(width, height);
    /// img.code39_gen("*PINEAPPLE*");
    /// ```
    fn code39_gen(&mut self, input: &str) -> Result<Self, BarcodeError> where Self: Sized;
    // fn code39_read(path: &str) -> String;
    // fn code128_gen(&mut self, input: &str, scale: T) -> Self;
}

impl BarcodeGen for Image {
    fn code39_gen(&mut self, input: &str) -> Result<Self, BarcodeError> {
        // Image stats
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut white: bool = false;

        // This loop codes a single character [ch] from input
        // with a 1px wide white line after
        for ch in input.chars() {
            // Get the code for encoding [ch]
            let code = match CODES39.get_by_left(&ch) {
                Some(char) => *char,
                None => return Err(BarcodeError::InvalidCharacter(ch.to_string()))
            };

            for thickness in code {
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

        Ok(self.clone())
    }

    // fn code39_read(path: &str) -> String {
    //     let img = match bmp::open(path) {
    //         Ok(img) => img,
    //         Err(e) => panic!("{}", e)
    //     };
    // }
}