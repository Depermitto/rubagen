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

    // Get scale of the barcode
    let scale = Text::new("Scale: ")
        .with_default("1")
        .prompt()
        .unwrap();

    // Set up image dimensions
    let scale: u32 = scale.trim().parse().unwrap(); // Get the string value
    let height = scale * 16; // Set height
    let width = scale * input.len() as u32 * 13; // Set width to input width * 13

    // Generate the image
    let mut img = Image::new(width, height);
    img.code39_gen(&input, scale as usize);

    img.save("result.bmp").expect("Couldn't save file");
}