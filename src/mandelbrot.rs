use image::Rgb;
use num::complex::Complex;
use palette::{Gradient, LinSrgb};

const ITERATIONS: i32 = 500;
const GRADIENT: [(f64, (u8, u8, u8)); 5] = [
    (0.0, (0, 7, 100)),
    (0.16, (32, 107, 203)),
    (0.42, (237, 255, 255)),
    (0.6425, (255, 170, 0)),
    (0.8575, (0, 2, 0)),
];

fn get_gradient_color(index: i32) -> Rgb<u8> {
    let iterations = ITERATIONS as f64;
    let gradient = Gradient::with_domain(
        GRADIENT
            .iter()
            .cloned()
            .map(|(scalar, (r, g, b))| {
                (
                    scalar * iterations,
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

pub fn mandelbrot(re: f64, im: f64) -> Rgb<u8> {
    let c0 = Complex::new(re, im);
    let mut z = Complex::new(0_f64, 0_f64);

    for i in 0..ITERATIONS {
        if z.norm() <= 2.0 {
            z = z * z + c0;
        } else {
            return get_gradient_color(i);
        }
    }

    Rgb([0, 0, 0])
}
