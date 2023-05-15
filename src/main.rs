use bmp::Image;
use inquire::{Text, formatter::StringFormatter};

mod ops;
mod data;

use ops::BarcodeGen;


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

    // Set up image dimensions
    let width = input.len() as u32 * 13; // Set width to input width * 13
    let height = 16; // Set height

    // Generate the image
    let mut img = Image::new(width, height);
    img.code39_gen(&input);

    img.save("result.bmp").expect("Couldn't save file");
}