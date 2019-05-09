
pub mod image_converter {
    extern crate image;
    use std::char;
    use image::{DynamicImage};
    use image::{GenericImageView};
    use std::fs::File;
    use std::io::{Write, Error};

    pub trait PixelConverter {
        fn convert(&self, data: [u8; 4]) -> char;
    }

    pub trait OutputImageStream {
        fn new(width: u32, height: u32) -> Self;

        fn on_start(&mut self){}
        fn add(&mut self, h: u32, w: u32, pixel: char);
        fn on_end(&mut self){}
    }

    pub trait ImageProcessor<T> {
        fn process_image<TConverter: PixelConverter>(&mut self, img: DynamicImage, converter: TConverter);
    }

    impl<T: OutputImageStream> ImageProcessor<T> for T {
        fn process_image<TConverter: PixelConverter>(&mut self, img: DynamicImage, converter: TConverter) {
            self.on_start();
            for (x, y, pixel) in img.grayscale().pixels() {
                let char_color = converter.convert(pixel.data);

                self.add(y, x, char_color);

            }
            self.on_end();
        }
    }

    pub struct ConsoleStream;

    impl OutputImageStream for ConsoleStream {
        fn new(_width: u32, _: u32) -> ConsoleStream {
            ConsoleStream
        }

        fn add(&mut self, _h: u32, w: u32, pixel: char) {
            print!("{}", pixel);
            if w == 0 {
                println!();
            }
        }
    }

    pub struct ImageFileStream {
        file: Option<File>,
        line: String
    }

    impl ImageFileStream {
        pub fn create_file(&mut self, path: &str) -> Option<Error> {
            let create_open = File::create(path);
            match create_open {
                Ok(file) => {
                    self.file = Some(file);
                    None
                },
                Err(err) => Some(err)
            }
        }
    }

    impl OutputImageStream for ImageFileStream {
        fn new(width: u32, height: u32) -> Self {
            ImageFileStream {
                file: None,
                line: String::with_capacity(width as usize * height as usize + height as usize)
            }
        }

        fn add(&mut self, _h: u32, w: u32, pixel: char) {
            if w == 0 && self.line.len() > 0 {
                self.line.push('\n');
            }

            self.line.push(pixel);
        }

        fn on_end(&mut self) {
            match &mut self.file {
                Some(f) => {
                    let bytes = self.line.as_bytes();
                    let _ = f.write_all(bytes);
                }
                None => panic!()
            }
        }
    }

    pub struct AsciiPixelConverter {
        data: Vec<char>
    }

    impl AsciiPixelConverter {
        pub fn new(data: Vec<char>) -> Self {
            AsciiPixelConverter {
                data
            }
        }
    }

    impl PixelConverter for AsciiPixelConverter {
        fn convert(&self, x: [u8; 4]) -> char {
            let x = x[0..3].iter().map(|x| *x as u32).sum::<u32>() / 3;
            safe_extract(&self.data, x as usize)
        }
    }

    fn safe_extract(data: &Vec<char>, idx: usize) -> char {
        data[idx % data.len()]
    }
}
