extern crate image;

use image::RgbImage;
use std::path::PathBuf;
use std::sync::mpsc;
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
    output: PathBuf,
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
    let mut img = RgbImage::new(size.width, size.height);

    let now = Instant::now();

    let scale_x = (rect.a2 - rect.a1) / (size.width as f64 - 1.0);
    let scale_y = (rect.b2 - rect.b1) / (size.height as f64 - 1.0);

    let chunk_size = match size.height as usize / threads / threads {
        0 => 1,
        res => res,
    };

    let band_heights: Vec<u32> = (0..size.height).collect::<Vec<u32>>();
    let bands_iter = band_heights
        .chunks(chunk_size)
        .map(move |band| {
            band.into_iter()
                .map(|h| {
                    (0..size.width)
                        .map(move |w| (*h, w))
                        .collect::<Vec<(u32, u32)>>()
                })
                .flatten()
                .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();

    let (tx, rx) = mpsc::channel::<(u32, u32, image::Rgb<u8>)>();

    let tx_arc = Arc::new(tx);
    let bands_arc = Arc::new(Mutex::new(bands_iter.into_iter()));
    for _ in 0..threads {
        let bands_clone = Arc::clone(&bands_arc);
        let sender = mpsc::Sender::clone(&tx_arc);
        thread::spawn(move || {
            let bands_iter = &*bands_clone;
            loop {
                let band = bands_iter.lock().unwrap().next();
                if band.is_none() {
                    break;
                }
                for (y, x) in band.unwrap() {
                    let re = x as f64 * scale_x + rect.a1;
                    let im = y as f64 * scale_y + rect.b1;
                    sender
                        .send((x as u32, size.height - 1 - y as u32, mandelbrot(re, im)))
                        .unwrap();
                }
            }
            drop(sender);
        });
    }
    drop(tx_arc);

    for pixel in rx {
        img.put_pixel(pixel.0, pixel.1, pixel.2);
    }

    println!("Image buffer filled: {}ms", now.elapsed().as_millis());

    img.save(file_name).unwrap();

    println!("Image write complete: {}ms", now.elapsed().as_millis());
}
