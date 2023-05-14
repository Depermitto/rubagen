use bmp::{Image, consts::WHITE};
use inquire::{Text, formatter::StringFormatter};

use data::CODES;
mod data;

fn main() {
    let formatter: StringFormatter = &|s| {
        format!("*{}*", s.trim().to_uppercase())
    };

    // Get the text to encode
    let input = Text::new("Text to encode: ")
        .with_formatter(formatter)
        .with_default("Example")
        .prompt()
        .unwrap();
    let input = formatter(&input);

    // Get scale of the barcode
    let scale = Text::new("Scale: ")
        .with_default("1")
        .prompt()
        .unwrap();

    // Set up image dimensions
    let scale: u32 = scale.trim().parse().unwrap(); // Get the string value
    let height = scale * 16; // Set height
    let width = scale * input.len() as u32 * 13; // Set width to input width * 13

    // Image stats
    let mut img = Image::new(width, height);
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut white: bool = false;

    for ch in input.chars() {
        // This loop codes a single character [ch] from input
        // with a 1px wide white line after
        for mut thickness in CODES[&ch] {
            // Repeat [thickness] times
            thickness *= scale as usize;
            while thickness != 0 {
                // Paint vertical line white
                while white && y < height {
                    img.set_pixel(x, y, WHITE);
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

    img.save("result.bmp").expect("Couldn't save file");
}