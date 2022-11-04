use num::complex::Complex;
use palette::{
    encoding::{Linear, Srgb},
    rgb::Rgb,
    Gradient,
};

fn get_gradient_color(
    index: u32,
    gradient: &Gradient<Rgb<Linear<Srgb>>, Vec<(f32, Rgb<Linear<Srgb>>)>>,
) -> [u8; 3] {
    let color = gradient.get(index as f32);

    [
        (color.red * 255.0) as u8,
        (color.green * 255.0) as u8,
        (color.blue * 255.0) as u8,
    ]
}

pub fn mandelbrot(
    re: f64,
    im: f64,
    max_iter: u32,
    gradient: &Gradient<Rgb<Linear<Srgb>>, Vec<(f32, Rgb<Linear<Srgb>>)>>,
) -> [u8; 3] {
    let c0 = Complex::new(re, im);
    let mut z = Complex::new(0_f64, 0_f64);

    for i in 0..max_iter {
        if z.norm_sqr() <= 4.0 {
            z = z * z + c0;
        } else {
            return get_gradient_color(i, gradient);
        }
    }

    [0, 0, 0]
}
