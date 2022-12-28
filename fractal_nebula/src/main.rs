mod paths;

use color_palette::Palette;
use rusty_fractals_core::mem::Mem;
use rusty_fractals_core::machine::Machine;
use rusty_fractals_core::fractal::{AppConfig, CalculationConfig, AreaDomainConfig, Math, ResultConfig};
use rusty_fractals_domain::{domain_area, resolution_multiplier};
use rusty_fractals_result::palette;
use rusty_fractals_result::palettes::PALETTE_BLUE_TO_WHITE;
use resolution_multiplier::ResolutionMultiplier;
use resolution_multiplier::ResolutionMultiplier::SquareAlter;
use rusty_fractals_core::machine;
use rusty_fractals_domain::domain::Domain;

struct Nebula {
    pub name: String,
}

impl Math for Nebula {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.plus(origin_re, origin_im);
    }
}

fn main() {
    let name = "Nebula";

    let calculation_config = CalculationConfig {
        iteration_min: 42,
        iteration_max: 14800,
    };
    let result_config = ResultConfig {
        palette: PALETTE_BLUE_TO_WHITE
    };
    let app_config = AppConfig {
        repeat: false,
        save_images: false,
    };
    let area_domain_config = AreaDomainConfig {
        width_re: 7.0,
        center_re: 0.0,
        center_im: 0.0,
        width_x: 1280,
        height_y: 720,
        resolution_multiplier: SquareAlter,
    };

    println!("Fractal {}", name);


    let area = domain_area::init(area_domain_config);
    let domain = Domain {
        width: area.width_x,
        height: area.height_y,
        domain_area: area,
        domain_elements: init_domain_elements(),
    };
    let machine = machine::Machine {
        domain,
        calculation_config,
    };

    machine.calculate();

    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);

    println!("Finished.");
}


#[test]
fn test_math() {
    let nebula = Nebula { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    nebula.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
