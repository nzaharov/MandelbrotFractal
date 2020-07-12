#[macro_use]
extern crate log;
extern crate image;

use image::RgbImage;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;
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
    #[structopt(short, long, default_value = "12")]
    gran: usize,
    #[structopt(short, long, default_value = "zad15.png")]
    output: PathBuf,
    #[structopt(short = "q", long = "quiet")]
    is_quiet: bool,
    #[structopt(short = "i", long = "iter", default_value = "1000")]
    max_iter: u32,
}

fn main() {
    let args = Cli::from_args();
    let Cli {
        rect,
        size,
        output: file_name,
        threads,
        is_quiet,
        max_iter,
        gran,
    } = args;

    if !is_quiet {
        simple_logger::init().unwrap();
    }

    info!("Starting...");
    let now = Instant::now();

    let scale_x = (rect.a2 - rect.a1) / (size.width as f64 - 1.0);
    let scale_y = (rect.b2 - rect.b1) / (size.height as f64 - 1.0);

    let chunk_size = match size.height as usize / (gran * threads) {
        0 => 1,
        res => res,
    };
    let chunks = (0..size.height)
        .collect::<Vec<u32>>()
        .chunks(chunk_size)
        .map(|band| {
            band.iter()
                .map(|h| {
                    (0..size.width)
                        .map(|w| (*h, w))
                        .collect::<Vec<(u32, u32)>>()
                })
                .flatten()
                .collect::<Vec<(u32, u32)>>()
        })
        .enumerate()
        .fold(vec![Vec::new(); threads], |mut acc, (i, band)| {
            acc.get_mut(i % threads).unwrap().extend(band);
            acc
        });
    info!("Bands prepared");

    let (tx, rx) = mpsc::channel::<Vec<(u32, u32, image::Rgb<u8>)>>();

    let tx_arc = Arc::new(tx);
    for (thread_id, chunk) in chunks.into_iter().enumerate() {
        let sender = mpsc::Sender::clone(&tx_arc);
        thread::Builder::new()
            .name(thread_id.to_string())
            .spawn(move || {
                info!(
                    "Worker thread {} starting...",
                    thread::current().name().unwrap()
                );
                let pixels = chunk
                    .into_iter()
                    .map(|(y, x)| {
                        let re = x as f64 * scale_x + rect.a1;
                        let im = y as f64 * scale_y + rect.b1;
                        (x, size.height - 1 - y, mandelbrot(re, im, max_iter))
                    })
                    .collect();

                sender.send(pixels).unwrap();
                drop(sender);
                info!(
                    "Worker thread {} exiting...",
                    thread::current().name().unwrap()
                );
            })
            .unwrap();
    }
    drop(tx_arc);

    let mut img = RgbImage::new(size.width, size.height);
    for pixels in rx {
        for pixel in pixels {
            img.put_pixel(pixel.0, pixel.1, pixel.2);
        }
    }

    info!("Image buffer filled: {}ms", now.elapsed().as_millis());
    img.save(file_name).unwrap();

    info!(
        "Image write complete: {}ms. Exiting...",
        now.elapsed().as_millis()
    );
}
