use bmp::Image;
use inquire::{Text, formatter::StringFormatter};

mod ops;
mod data;
mod error;

use ops::BarcodeGen;


fn main() {
    let formatter: StringFormatter = &|s| {
        format!("*{}*", s.trim().to_uppercase())
    };

    'main: loop {
        // Get the text to encode
        let msg = "Text to encode: (CTRL+C to exit)";
        let input = Text::new(msg)
            .with_formatter(formatter)
            .prompt();

        let input = match input {
            Ok(string) => formatter(&string),
            Err(_) => break 'main
        };

        // Set up image dimensions
        let width = input.len() as u32 * 16; // Set width to input width * 13
        let height = width / 4; // Set height

        // Generate the image
        let mut img = Image::new(width, height);
        // Handle possible errors
        match img.code39_gen(&input) {
            Ok(()) => println!("Operation successful!\nImage saved to \"result.bmp\""),
            Err(e) => {
                println!("{}\nPlease try again", e);
                continue 'main;
            }
        }

        img.save("result.bmp").expect("Couldn't save file");
    }
}