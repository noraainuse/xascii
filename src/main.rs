use crossterm::{terminal, ExecutableCommand};
use ffmpeg_next as ffmpeg;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer, Rgb, Rgba};
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_size::terminal_size;

fn main() {
    println!("Enter the path to the file you want to convert (image or video):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input_path = input.trim();

    create_output_directory(); // Create the output folder

    if input_path.ends_with(".mp4") || input_path.ends_with(".avi") || input_path.ends_with(".mkv")
    {
        process_video(input_path);
    } else {
        process_image(input_path);
    }
}

fn process_image(input_path: &str) {
    let img = match image::open(input_path) {
        Ok(i) => i,
        Err(_) => {
            eprintln!("Failed to open the image file.");
            return;
        }
    };

    let term_height = terminal_size().map(|(_, h)| h.0 as u32).unwrap_or(50);

    let character_aspect_ratio = 2.5;
    let target_height = term_height.min(100);
    let target_width = (target_height as f32 * character_aspect_ratio) as u32;

    let resized_img = img.resize_exact(target_width, target_height, FilterType::Lanczos3);
    display_ascii(&resized_img); // Display ASCII in the terminal
    save_ascii_to_file(input_path, &resized_img); // Save to a text file quietly
}

fn process_video(input_path: &str) {
    ffmpeg::init().expect("Failed to initialize ffmpeg");
    let mut ictx = ffmpeg::format::input(&input_path).expect("Failed to open video file");

    let video_stream_index = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .expect("Failed to find video stream")
        .index();

    let codec = ffmpeg::codec::context::Context::from_parameters(
        ictx.stream(video_stream_index).unwrap().parameters(),
    )
    .unwrap();
    let mut decoder = codec.decoder().video().unwrap();

    let term_height = terminal_size().map(|(_, h)| h.0 as u32).unwrap_or(50);

    let character_aspect_ratio = 2.5;
    let target_height = term_height.min(50);
    let target_width = (target_height as f32 * character_aspect_ratio) as u32;

    let mut frame_index = 0;
    std::io::stdout()
        .execute(terminal::EnterAlternateScreen)
        .unwrap();
    terminal::enable_raw_mode().unwrap();

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            let mut frame = ffmpeg::frame::Video::empty();
            while decoder.receive_frame(&mut frame).is_ok() {
                frame_index += 1;
                if frame_index % 2 != 0 {
                    continue; // Skip every second frame for performance
                }

                let img = frame_to_image(&frame);
                let resized_img = DynamicImage::ImageRgb8(img).resize_exact(
                    target_width,
                    target_height,
                    FilterType::Lanczos3,
                );
                display_ascii(&resized_img); // Display each frame in the terminal
                save_ascii_to_file(&format!("frame_{}.txt", frame_index), &resized_img); // Save the frame to a text file quietly
                std::thread::sleep(Duration::from_millis(33)); // Simulate ~30 FPS
            }
        }
    }

    std::io::stdout()
        .execute(terminal::LeaveAlternateScreen)
        .unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn frame_to_image(frame: &ffmpeg::frame::Video) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = frame.width();
    let height = frame.height();
    let mut buf = vec![0; (width * height * 3) as usize];
    let data = frame.data(0); // This directly gets the frame data.
    let stride = frame.stride(0);
    for (i, chunk) in buf.chunks_mut(3).enumerate() {
        let y = i / width as usize;
        let x = i % width as usize;
        let index = y * stride + x * 3;
        if index + 2 < data.len() {
            chunk[0] = data[index]; // Red
            chunk[1] = data[index + 1]; // Green
            chunk[2] = data[index + 2]; // Blue
        }
    }

    ImageBuffer::from_vec(width, height, buf).expect("Failed to create ImageBuffer")
}

fn save_ascii_to_file(input_path: &str, img: &DynamicImage) {
    let (ascii_art, _) = image_to_ascii_with_color(img);

    let output_filename = get_output_filename(input_path);
    let file_path = Path::new("ascii-results").join(output_filename);

    let mut file = File::create(file_path).expect("Failed to create file");

    for line in ascii_art {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}

fn get_output_filename(input_path: &str) -> String {
    let path = Path::new(input_path);
    let file_stem = path.file_stem().unwrap().to_str().unwrap(); // Fixed type mismatch here
    let _extension = path
        .extension()
        .unwrap_or_else(|| std::ffi::OsStr::new(""))
        .to_str()
        .unwrap(); // Prefix with `_` to suppress warning

    // Replace the extension with ".txt"
    format!("{}.txt", file_stem)
}

fn create_output_directory() {
    let output_dir = Path::new("ascii-results");
    if !output_dir.exists() {
        create_dir_all(output_dir).expect("Failed to create output directory");
    }
}

fn display_ascii(img: &DynamicImage) {
    let (ascii_art, colors) = image_to_ascii_with_color(img);

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
    let chars = [" ", ".", "·", ":", "!", "~", "*", "=", "$", "#", "@"];

    let mut ascii_lines = Vec::new();
    let mut color_lines = Vec::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        let mut line = String::new();
        let mut color_line = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness =
                (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32)
                    / 255.0;
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
