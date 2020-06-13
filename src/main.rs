extern crate image;

use image::RgbImage;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
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
    threads: usize,
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
        threads,
        ..
    } = args;
    let img = RgbImage::new(size.width, size.height);

    let now = Instant::now();

    let scale_x = (rect.a2 - rect.a1) / (size.width as f64 - 1.0);
    let scale_y = (rect.b2 - rect.b1) / (size.height as f64 - 1.0);

    let band_heights = (0..size.height).collect::<Vec<u32>>();
    let bands_iter = band_heights
        .chunks(size.height as usize / threads)
        .map(|band| {
            band.iter()
                .map(|h| {
                    (0..size.width)
                        .map(move |w| (*h, w))
                        .collect::<Vec<(u32, u32)>>()
                })
                .flatten()
                .collect::<Vec<(u32, u32)>>()
        });

    let mut handles = vec![];
    let img_arc = Arc::new(Mutex::new(img));

    // for band in bands_iter {
    //     println!("{:?}", band);
    // }

    for band in bands_iter {
        let a1 = rect.a1;
        let b1 = rect.b1;
        let img_clone = Arc::clone(&img_arc);
        let handle = thread::spawn(move || {
            for (y, x) in band {
                let re = x as f64 * scale_x + a1;
                let im = y as f64 * scale_y + b1;
                img_clone
                    .lock()
                    .unwrap()
                    .put_pixel(x as u32, y as u32, mandelbrot(re, im));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Image buffer filled: {}ms", now.elapsed().as_millis());

    img_arc.lock().unwrap().save(file_name).unwrap();

    println!("Image write complete: {}ms", now.elapsed().as_millis());
}
