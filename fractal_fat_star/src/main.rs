use rusty_fractals_core::{machine, window};
use rusty_fractals_common::area::AreaConfig;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::fractal::{AppConfig, CalculationConfig, Fractal};
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_result::palettes::palette_blue_to_white_circle_up;
use rusty_fractals_result::result::ResultConfig;

struct FatStar {}

impl Fractal<Mem> for FatStar {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }
    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        // infinite orbits
        length > min && iterator == max
    }
}

fn main() {
    let name = "Fat Star";

    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 22000,
        resolution_multiplier: Single,
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_config = AreaConfig {
        width_re: 3.5,
        center_re: 0.0,
        center_im: 0.0,
        width_x: WIDTH,
        height_y: HEIGHT,
    };
    let result_config = ResultConfig {
        palette: palette_blue_to_white_circle_up(),
    };

    let fat_star = FatStar {};
    let machine = machine::init(&calculation_config, &app_config, &result_config, &area_config);
    let (domain_image, result_image) = machine.calculate(&fat_star);

    window::show(name, domain_image, &result_image);
}


#[test]
fn test_math() {
    let fat_star = FatStar {};
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}