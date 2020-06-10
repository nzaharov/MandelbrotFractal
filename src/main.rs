extern crate image;

use image::RgbImage;

use std::path::PathBuf;
use structopt::StructOpt;

mod image_size;
mod mandelbrot;
mod rect;
use crate::image_size::*;
use crate::mandelbrot::*;
use crate::rect::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "640x480")]
    size: ImageSize,
    #[structopt(short, long, default_value = "-2.0:2.0:-2.0:2.0")]
    rect: Rect,
    #[structopt(short, long, default_value = "1")]
    threads: u8,
    #[structopt(short, long, default_value = "zad15.png")]
    output: PathBuf, // TODO image format validation?
    #[structopt(short = "q", long = "quiet")]
    is_quiet: bool,
}

fn main() {
    let args = Cli::from_args();
    let Cli {
        rect,
        size,
        output: file_name,
        ..
    } = args;
    let mut img = RgbImage::new(size.width, size.height);

    for x in 0..size.width {
        for y in 0..size.height {
            img.put_pixel(x, y, mandelbrot(x, y, &rect, &size));
        }
    }

    img.save(file_name).unwrap();
}
