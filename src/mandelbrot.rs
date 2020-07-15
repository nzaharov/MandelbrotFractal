use image::Rgb;
use num::complex::Complex;
use palette::{Gradient, LinSrgb};

const GRADIENT: [(f64, (u8, u8, u8)); 5] = [
    (0.0, (0, 7, 100)),
    (0.16, (32, 107, 203)),
    (0.42, (237, 255, 255)),
    (0.6425, (255, 170, 0)),
    (0.8575, (0, 2, 0)),
];

fn get_gradient_color(index: u32, max_iter: u32) -> Rgb<u8> {
    let gradient = Gradient::with_domain(
        GRADIENT
            .iter()
            .cloned()
            .map(|(scalar, (r, g, b))| {
                (
                    scalar * max_iter as f64,
                    LinSrgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0),
                )
            })
            .collect(),
    );

    let color = gradient.get(index as f64);

    Rgb([
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ])
}

pub fn mandelbrot(re: f64, im: f64, max_iter: u32) -> Rgb<u8> {
    let c0 = Complex::new(re, im);
    let mut z = Complex::new(0_f64, 0_f64);

    for i in 0..max_iter {
        if z.norm_sqr() <= 2500.0 {
            z = z.exp() - c0;
        } else {
            return get_gradient_color(i, max_iter);
        }
    }

    Rgb([0, 0, 0])
}
