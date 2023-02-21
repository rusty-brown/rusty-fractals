use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{CalculationConfig, Fractal, FractalMath};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_result::result::ResultConfig;

struct Infinity {}

impl FractalMath<Mem> for Infinity {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for Infinity {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::infinite_orbits(min, max, length, iterator)
    }
    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
        fractal::calculate_path(self, self, area, iteration_min, iteration_max, origin_re, origin_im, result_static)
    }
}


fn main() {
    let name = "Infinity";

    const WIDTH: usize = 1000; // 1920
    const HEIGHT: usize = 1000; // 1080

    let calculation_config = CalculationConfig {
        iteration_min: 3000,
        iteration_max: 80_000,
        resolution_multiplier: Single,
    };
    let area_config = AreaConfig {
        width_re: 2.6,
        center_re: -0.5,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let infinity = Infinity {};
    let machine = machine::init(&calculation_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&infinity);

    window::show(name, domain_image, &result_image);
}

#[test]
fn test_math() {
    let infinity = Infinity {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    infinity.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
