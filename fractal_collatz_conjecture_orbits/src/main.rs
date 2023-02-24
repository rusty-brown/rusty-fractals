use rusty_fractals_core::machine;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath};
use rusty_fractals_common::mem_collatz::MemCollatz;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square11;

struct CollatzConjectureOrbits {
    name: &'static str,
}

impl FractalMath<MemCollatz> for CollatzConjectureOrbits {
    fn math(&self, m: &mut MemCollatz, origin_re: f64, origin_im: f64) {
        m.square();
        m.collatz_conjecture();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for CollatzConjectureOrbits {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    const WIDTH: usize = 1280;
    const HEIGHT: usize = 720;
    let fractal_config = FractalConfig {
        iteration_min: 7,
        iteration_max: 1348,
        resolution_multiplier: Square11,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_re: 5.0,
        center_re: -0.088485445553580480,
        center_im: -0.200679435068532800,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let collatz = &CollatzConjectureOrbits { name: "Collatz Conjecture Orbits" };
    machine::nebula_calculation_for(collatz, WIDTH, HEIGHT, fractal_config, area_config);
}

#[cfg(test)]
mod tests {
    use rusty_fractals_common::fractal::FractalMath;
    use rusty_fractals_common::mem::Mem;
    use rusty_fractals_common::mem_collatz::MemCollatz;
    use crate::CollatzConjectureOrbits;

    #[test]
    fn test_math() {
        let collatz = CollatzConjectureOrbits { name: "Collatz orbits" };
        let mut mc = MemCollatz { m: Mem { re: 0.0, im: 0.0 }, num: 7 };
        collatz.math(&mut mc, 1.0, 0.1);
        assert_eq!(mc.re(), 2.0);
        assert_eq!(mc.im(), 1.1);
    }
}
