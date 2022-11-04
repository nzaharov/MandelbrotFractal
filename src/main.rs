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

fn run(rect: Rect, size: ImageSize, threads: usize, max_iter: u32, gran: usize) -> Vec<u8> {
    info!("Starting...");

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
                .cloned()
                .map(|h| (h, (0..size.width).map(|w| (h, w)).collect()))
                .collect::<Vec<(u32, Vec<(u32, u32)>)>>()
        })
        .enumerate()
        .fold(vec![Vec::new(); threads], |mut acc, (i, band)| {
            acc.get_mut(i % threads).unwrap().extend(band);
            acc
        });
    info!("Bands prepared");

    let gradient = get_gradient(max_iter);

    let mut lines = chunks
        .into_par_iter()
        .enumerate()
        .flat_map(|(i, chunk)| {
            info!("Worker thread {} starting...", i);

            let subpixel_lines = chunk
                .into_iter()
                .map(|(h, line)| {
                    (
                        h,
                        line.into_iter()
                            .map(|(y, x)| {
                                (x as f64 * scale_x + rect.a1, y as f64 * scale_y + rect.b1)
                            })
                            .flat_map(|(re, im)| mandelbrot(re, im, max_iter, &gradient))
                            .collect(),
                    )
                })
                .collect::<Vec<(u32, Vec<u8>)>>();

            info!("Worker thread {} exiting...", i);
            subpixel_lines
        })
        .collect::<Vec<(u32, Vec<u8>)>>();

    lines.sort_by_cached_key(|line| line.0);

    lines.into_iter().flat_map(|line| line.1).collect()
}

fn main() {
    let args = Cli::load();
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

    let now = Instant::now();

    let subpixels = create_pool(threads)
        .unwrap()
        .install(|| run(rect, size, threads, max_iter, gran));

    let img = RgbImage::from_vec(size.width, size.height, subpixels).unwrap();
    info!("Image buffer filled: {}ms", now.elapsed().as_millis());
    img.save(file_name).unwrap();
    info!(
        "Image write complete: {}ms. Exiting...",
        now.elapsed().as_millis()
    );
}
