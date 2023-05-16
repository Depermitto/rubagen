use bmp::{Image, consts::WHITE};
use crate::{
    error::BarcodeError,
    data::CODES39,
};

pub trait BarcodeGen {
    /// Modifies `Self` with an encoded \[`input`] using
    /// [Code 39](https://en.wikipedia.org/wiki/Code_39).
    ///
    /// Returns `Ok(())` if the operation was a success,
    /// otherwise `Err(BarcodeError)`
    ///
    /// # Example
    ///
    /// basic usage:
    /// ```
    /// let mut img = Image::new(width, height);
    /// img.code39_gen("*PINEAPPLE*");
    /// ```
    fn code39_gen(&mut self, input: &str) -> Result<(), BarcodeError> where Self: Sized;
    // fn code39_read(path: &str) -> String;
    // fn code128_gen(&mut self, input: &str, scale: T) -> Self;
}

impl BarcodeGen for Image {
    fn code39_gen(&mut self, input: &str) -> Result<(), BarcodeError> {
        // Image stats
        let mut x: u32 = 0;
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
                if white {
                    // Repeat [thickness] times
                    for i in 0..thickness {
                        // Paint a single vertical line white
                        for y in 0..self.get_height() {
                            self.set_pixel(x + i, y, WHITE);
                        }
                    }
                }
                x += thickness;
                // Switch colors
                white = !white;
            }
        }

        Ok(())
    }

    // fn code39_read(path: &str) -> String {
    //     let img = match bmp::open(path) {
    //         Ok(img) => img,
    //         Err(e) => panic!("{}", e)
    //     };
    // }
}