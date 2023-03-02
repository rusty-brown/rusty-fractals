use rusty_fractals_core::{machine};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::data_image::DataImage;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{FractalConfig, Fractal, FractalMath, FractalName, Recalculate};
use rusty_fractals_common::fractal;
use rusty_fractals_common::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square9;

struct Nebula {}

impl FractalMath<Mem> for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for Nebula {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, data: &DataImage, is_wrap: bool) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, data, is_wrap)
    }
}

impl FractalName for Nebula {
    fn name(&self) -> &'static str { "Nebula" }
}

impl Recalculate for Nebula {
    fn recalculate() { todo!() }
}

fn main() {
    let fractal_config = FractalConfig {
        iteration_min: 42,
        iteration_max: 14800,
        resolution_multiplier: Square9,
        palette: palette_blue_to_white_circle_up(),
    };
    let area_config = AreaConfig {
        width_x: 1280,
        height_y: 1000,
        width_re: 3.5,
        center_re: -0.5,
        center_im: 0.0,
    };
    let nebula = &Nebula {};
    machine::nebula_calculation_for(nebula, fractal_config, area_config);
}

#[test]
fn test_math() {
    let nebula = Nebula {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
