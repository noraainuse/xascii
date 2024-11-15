use std::io::{self, Write};
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut input = String::new();
    println!("Enter the path to the image you want to convert:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_path = input.trim();

    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(_) => {
            eprintln!("Failed to open the image file.");
            return;
        }
    };

    let resized_img = img.resize(100, 50, FilterType::Nearest);
    let ascii_art = image_to_ascii(&resized_img);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for line in ascii_art {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
        writeln!(&mut stdout, "{}", line).unwrap();
    }
}

fn image_to_ascii(img: &DynamicImage) -> Vec<String> {
    let chars = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];
    let mut ascii_lines = Vec::new();
    let (width, height) = img.dimensions();
    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let intensity = pixel[0] as f64 / 255.0;
            let index = (intensity * (chars.len() - 1) as f64).round() as usize;
            line.push_str(chars[index]);
        }
        ascii_lines.push(line);
    }
    ascii_lines
}
