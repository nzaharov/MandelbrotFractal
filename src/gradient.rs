use palette::rgb;
use palette::Gradient;
use palette::LinSrgb;

const GRADIENT: [(f64, (u8, u8, u8)); 5] = [
    (0.0, (0, 7, 100)),
    (0.16, (32, 107, 203)),
    (0.42, (237, 255, 255)),
    (0.6425, (255, 170, 0)),
    (0.8575, (0, 2, 0)),
];

pub fn get_gradient(
    max_iter: u32,
) -> Gradient<
    rgb::Rgb<palette::encoding::Linear<palette::encoding::Srgb>>,
    Vec<(
        f32,
        rgb::Rgb<palette::encoding::Linear<palette::encoding::Srgb>>,
    )>,
> {
    Gradient::with_domain(
        GRADIENT
            .iter()
            .cloned()
            .map(|(s, (r, g, b))| {
                (
                    s as f32,
                    (rgb::Rgb::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)),
                )
            })
            .map(|(scalar, color)| (scalar * max_iter as f32, LinSrgb::from(color)))
            .collect::<Vec<(f32, LinSrgb)>>(),
    )
}
