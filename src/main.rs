extern crate image;

use image::RgbImage;

use std::path::PathBuf;
use std::time::Instant;
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

    let now = Instant::now();

    for x in 0..size.width {
        for y in 0..size.height {
            let re = x as f64 * (rect.a2 - rect.a1) / (size.width as f64 - 1.0) + rect.a1;
            let im = y as f64 * (rect.b2 - rect.b1) / (size.height as f64 - 1.0) + rect.b2;
            img.put_pixel(x, y, mandelbrot(re, im));
        }
    }

    println!("Image buffer filled: {}s", now.elapsed().as_millis());

    img.save(file_name).unwrap();

    println!("Image write complete: {}s", now.elapsed().as_millis());
}
