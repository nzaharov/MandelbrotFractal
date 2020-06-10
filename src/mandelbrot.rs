use image::Rgb;
use num::complex::Complex;

use crate::image_size::*;
use crate::rect::*;

fn check_in_area(a: f32, b: f32, rect: &Rect) -> bool {
    a >= rect.a1 && a <= rect.a2 && b >= rect.b1 && b <= rect.b2
}

pub fn mandelbrot(x: u32, y: u32, rect: &Rect, size: &ImageSize) -> Rgb<u8> {
    let x = x as f32 * (rect.a2 - rect.a1) / (size.width as f32 - 1.0) + rect.a1;
    let y = y as f32 * (rect.b2 - rect.b1) / (size.height as f32 - 1.0) + rect.b1;
    let c0 = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for _ in 0..1000 {
        if check_in_area(z.re, z.im, &rect) {
            z = z * z + c0;
        } else {
            return Rgb([255, 255, 255]);
        }
    }

    Rgb([0, 0, 0])
}
