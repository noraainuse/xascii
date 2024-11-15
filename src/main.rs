use std::io::{self, Write};
use image::{DynamicImage, GenericImageView, imageops::FilterType, Rgba};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_size::{terminal_size, Height};

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

    let term_height = terminal_size()
        .map(|(_, h)| h.0 as u32)
        .unwrap_or(50);

    let character_aspect_ratio = 2.5;
    let target_height = term_height.min(100);
    let target_width = (target_height as f32 * character_aspect_ratio) as u32;

    let resized_img = img.resize(target_width, target_height, FilterType::Lanczos3);
    let (ascii_art, colors) = image_to_ascii_with_color(&resized_img);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for (line, color_line) in ascii_art.iter().zip(colors.iter()) {
        for (ch, color) in line.chars().zip(color_line.iter()) {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(rgb_to_termcolor(color))))
                .unwrap();
            write!(&mut stdout, "{}", ch).unwrap();
        }
        writeln!(&mut stdout).unwrap();
    }
}

fn image_to_ascii_with_color(img: &DynamicImage) -> (Vec<String>, Vec<Vec<Rgba<u8>>>) {
    let chars = [
        " ", ".", "·", ":", "!", "~", "*", "=", "$", "#", "@"
    ];
    
    let mut ascii_lines = Vec::new();
    let mut color_lines = Vec::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        let mut line = String::new();
        let mut color_line = Vec::new();
        
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = (0.2126 * pixel[0] as f32 + 
                            0.7152 * pixel[1] as f32 + 
                            0.0722 * pixel[2] as f32) / 255.0;
            let gamma = 1.2;
            let corrected_brightness = brightness.powf(gamma);
            let index = ((1.0 - corrected_brightness) * (chars.len() - 1) as f32).round() as usize;
            line.push_str(chars[index]);
            color_line.push(pixel);
        }
        
        ascii_lines.push(line);
        color_lines.push(color_line);
    }
    
    (ascii_lines, color_lines)
}

fn rgb_to_termcolor(rgba: &Rgba<u8>) -> Color {
    Color::Rgb(rgba[0], rgba[1], rgba[2])
}
