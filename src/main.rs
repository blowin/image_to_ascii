extern crate image;

mod image_converter;

use crate::image_converter::image_converter::{ConsoleStream, OutputImageStream, ImageFileStream};
use crate::image_converter::image_converter::{ImageProcessor};
use crate::image_converter::image_converter::{AsciiPixelConverter};

use image::FilterType;
use clap::{Arg, App};

fn main() {
    use image::{GenericImageView};

    let matches = App::new("Image converter")
        .version("0.1.0")
        .author("Blowin <https://github.com/blowin>")
        .about("Image to ASCII converter")
        .arg(Arg::with_name("Image path")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Path to convert image"))
        .arg(Arg::with_name("Save path")
            .required(false)
            .takes_value(true)
            .index(2)
            .help("Path to save image"))
        .arg(Arg::with_name("Width")
            .short("W")
            .long("width")
            .takes_value(true)
            .min_values(0)
            .help("Width for resize"))
        .arg(Arg::with_name("Height")
            .short("H")
            .long("height")
            .takes_value(true)
            .min_values(0)
            .help("Height for resize"))
        .arg(Arg::with_name("Characters")
            .short("ch")
            .long("CH")
            .takes_value(true)
            .value_delimiter(",")
            .help("Values for replace"))
        .get_matches();

    let path = matches.value_of("Image path").unwrap();
    let width = matches.value_of("Width").unwrap_or("0").parse::<u32>();
    let width = match width {
        Ok(w) => w,
        Err(_err) => {
            println!("Invalid width param value");
            return;
        }
    };

    let height = matches.value_of("Height").unwrap_or("0").parse::<u32>();
    let height = match height {
        Ok(h) => h,
        Err(_) => {
            println!("Invalid height param value");
            return;
        }
    };

    let save_path = matches.value_of("Save path");
    let values = matches.values_of("Characters");
    let values = match values {
        Some(vals) => {
            vals
                .filter(|x| (**x).len() > 0)
                .map(|x| x.chars().next().unwrap())
                .collect()
        },
        None => vec!['#', '@', '!', '&', '?', '=', '+', '-', '.', ' ', '*', '%', ',', '/', ':', '~', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '{', '}', '|']
    };

    let img = image::open(path);
    match img {
        Ok(dyn_img) => {
            let width = if width == 0 {dyn_img.width()} else {width};
            let height = if height == 0 {dyn_img.height()} else {height};

            let dyn_img = dyn_img.resize(width, height, FilterType::Gaussian);

            let converter = AsciiPixelConverter::new(values);

            match save_path {
                Some(save_path) => {
                    let mut stream = ImageFileStream::new(width, height);
                    if let Some(err) = stream.create_file(save_path) {
                        println!("Error: {}", err.to_string());
                        return;
                    }

                    stream.process_image(dyn_img, converter);
                },
                None => {
                    let mut stream = ConsoleStream::new(width, height);
                    stream.process_image(dyn_img, converter);
                }
            }
        },
        Err(er) => println!("Error: {}", er.to_string())
    }
}
