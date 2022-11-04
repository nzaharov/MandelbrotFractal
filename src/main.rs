#[macro_use]
extern crate log;
extern crate image;

use image::RgbImage;
use image_size::ImageSize;
use rayon::prelude::*;
use rect::Rect;
use std::time::Instant;

mod cli;
mod gradient;
mod image_size;
mod mandelbrot;
mod rect;

use crate::cli::Cli;
use crate::gradient::get_gradient;
use crate::mandelbrot::*;

fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, rayon::ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
}

fn run(rect: Rect, size: ImageSize, chunk_size: usize, max_iter: u32) -> Vec<u8> {
    let scale_x = (rect.a2 - rect.a1) / (size.width as f64 - 1.0);
    let scale_y = (rect.b2 - rect.b1) / (size.height as f64 - 1.0);

    let gradient = get_gradient(max_iter);

    (0..size.height)
        .collect::<Vec<u32>>()
        .par_chunks(chunk_size)
        .flat_map(|band| {
            band.iter()
                .cloned()
                .flat_map(|line| {
                    (0..size.width)
                        .map(|w| (line, w))
                        .map(|(y, x)| (x as f64 * scale_x + rect.a1, y as f64 * scale_y + rect.b1))
                        .flat_map(|(re, im)| mandelbrot(re, im, max_iter, &gradient))
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>()
}

fn main() {
    let Cli {
        rect,
        size,
        output: file_name,
        threads,
        is_quiet,
        max_iter,
        gran,
    } = Cli::load();

    if !is_quiet {
        simple_logger::init().unwrap();
    }

    let now = Instant::now();
    info!("Starting...");

    let chunk_size = match size.height as usize / (gran * threads) {
        0 => 1,
        res => res,
    };

    let subpixels = create_pool(threads)
        .unwrap()
        .install(|| run(rect, size, chunk_size, max_iter));

    let img = RgbImage::from_vec(size.width, size.height, subpixels).unwrap();
    info!("Image buffer filled: {}ms", now.elapsed().as_millis());
    img.save(file_name).unwrap();
    info!(
        "Image write complete: {}ms. Exiting...",
        now.elapsed().as_millis()
    );
}
