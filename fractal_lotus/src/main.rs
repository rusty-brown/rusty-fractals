use rusty_fractals_core::{files, machine, window};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::area::{Area, AreaConfig};
use rusty_fractals_common::fractal;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal, MathMem};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Square11;
use rusty_fractals_common::result_data_static::ResultDataStatic;
use rusty_fractals_result::palettes::{palette_blue_to_white_circle_up};
use rusty_fractals_result::result::ResultConfig;

struct Lotus {}

impl MathMem for Lotus {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
}

impl Fractal for Lotus {
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        fractal::finite_orbits(min, max, length, iterator)
    }

    fn calculate_path(&self, area: &Area, iteration_min: u32, iteration_max: u32, origin_re: f64, origin_im: f64, result_static: &ResultDataStatic) -> (u32, u32) {
        fractal::calculate_path_mem(self, self, area, iteration_min, iteration_max, origin_re, origin_im, result_static)
    }
}

fn main() {
    let name = "Lotus";

    const WIDTH: usize = 1280;
    const HEIGHT: usize = 1000;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 8000,
        resolution_multiplier: Square11,
    };
    let area_config = AreaConfig {
        width_re: 3.5,
        center_re: 0.0, //  0.67748277351478,
        center_im: 0.0, // -1.18770078111202,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let lotus = Lotus {};
    let machine = machine::init(&calculation_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&lotus);

    window::show(name, domain_image, &result_image);
}