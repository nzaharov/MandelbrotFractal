use num::complex::Complex;
use palette::{rgb::Rgb, Hsv};

const ITERATIONS: i32 = 1000;

fn get_color(hue: i32) -> image::Rgb<u8> {
    let hue = hue % 360;
    let color = Hsv::new(hue as f32, 1.0, 1.0);
    let rgb: Rgb = Rgb::from(color);

    image::Rgb([
        (rgb.red * 255.0) as u8,
        (rgb.green * 255.0) as u8,
        (rgb.blue * 255.0) as u8,
    ])
}

pub fn mandelbrot(re: f64, im: f64) -> image::Rgb<u8> {
    let c0 = Complex::new(re, im);
    let mut z = Complex::new(0_f64, 0_f64);

    for i in 0..ITERATIONS {
        if z.norm() <= 2.0 {
            z = z * z + c0;
        } else {
            return get_color(i);
        }
    }

    image::Rgb([0, 0, 0])
}
