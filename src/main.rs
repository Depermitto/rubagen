use bmp::{Image, consts::WHITE};
use data::CODES;
use std::io::stdin;
mod data;

fn main() {
    // Get the text to encode
    let mut input= String::new();
    println!("Text: ");
    stdin()
        .read_line(&mut input)
        .expect("Could not read from user input");

    let input = input.trim().to_uppercase();

    // Get scale of the barcode
    let mut scale = String::from("1");
    println!("\n(1 means 18 px height and 1px thin lines)\nScale: ");
    stdin().read_line(&mut scale).expect("Could not read from user input");

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