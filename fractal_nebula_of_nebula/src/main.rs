use rusty_fractals::application;
use rusty_fractals::fractal::OrbitType::Finite;
use rusty_fractals::fractal::{FractalConfig, FractalMath};
use rusty_fractals::mem::Mem;
use rusty_fractals::palettes::palette_blue_to_white_circle_up;
use rusty_fractals::resolution_multiplier::ResolutionMultiplier::Square3;

pub struct NebulaOfNebula {}

impl FractalMath<Mem> for NebulaOfNebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        let r = m.re;
        let i = m.im;
        let or = origin_re;
        let oi = origin_im;
        // f(f(z)) : f = z^2 + c
        m.re = r * r * r * r - 6.0 * r * r * i * i + i * i * i * i + 2.0 * r * r * or
            - 2.0 * i * i * or
            - 4.0 * r * i * oi
            + or * or
            - oi * oi
            + or
            - r;
        m.im = 4.0 * r * r * r * i - 4.0 * r * i * i * i + 4.0 * r * i * or + 2.0 * r * r * oi
            - 2.0 * i * i * oi
            + 2.0 * or * oi
            + oi
            - i;
    }
}

fn main() {
    let fractal_config: FractalConfig = FractalConfig {
        name: "Nebula of Nebula",
        iteration_min: 42,
        iteration_max: 24800,
        resolution_multiplier: Square3,
        palette: palette_blue_to_white_circle_up(),

        width_x: 1280,
        height_y: 1000,
        width_re: 0.5,
        center_re: 0.0,
        center_im: 0.0,

        orbits: Finite,
        update_max: 150,
        update_min: 0,
    };

    let nebula_of_nebula = NebulaOfNebula {};

    // start program window
    let application = application::init(fractal_config);

    // execute calculation
    application.calculate(&nebula_of_nebula);
}

#[cfg(test)]
mod tests {
    use crate::NebulaOfNebula;
    use rusty_fractals::fractal::FractalMath;
    use rusty_fractals::mem::Mem;

    #[test]
    fn test_math() {
        let nebula_of_nebula = NebulaOfNebula {};
        let mut m = Mem { re: 0.0, im: 0.0 };
        nebula_of_nebula.math(&mut m, 1.0, 0.1);
        assert_eq!(m.re, 1.0);
        assert_eq!(m.im, 0.1);
    }
}
