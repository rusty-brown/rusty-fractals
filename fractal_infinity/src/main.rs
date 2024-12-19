use rusty_fractals::application;
use rusty_fractals::fractal::OrbitType::Infinite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Single;

pub struct Infinity {}

impl FractalMath<Mem> for Infinity {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let fractal_config: FractalConfig = FractalConfig {
        name: "Infinity",
        iteration_min: 3000,
        iteration_max: 30_000,
        resolution_multiplier: Single,
        palette: palette_blue_to_white_circle_up(),

        width_x: 600,
        height_y: 600,
        width_re: 2.6,
        center_re: -0.5,
        center_im: 0.0,

        orbits: Infinite,
        update_max: 150,
        update_min: 0,
    };

    let infinity = Infinity {};
    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&infinity);
}

#[cfg(test)]
mod tests {
    use crate::Infinity;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let infinity = Infinity {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        infinity.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
