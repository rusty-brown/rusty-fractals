use rusty_fractals_common::fractal::Fractal;
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier;
use rusty_fractals_common::resolution_multiplier::ResolutionMultiplier::Single;
use rusty_fractals_result::palette::Palette;
use rusty_fractals_result::palettes::palette_black_to_white_exp2;

const NAME: &str = "Fat Star Tentacle";

const ITERATION_MAX: u32 = 81_000;
const ITERATION_MIN: u32 = 8;
const AREA_SIZE: f64 = 3.5;
const TARGET_RE: f64 = 0.0;
const TARGET_IM: f64 = 0.0;
// TODO
// const INIT_FINEBROT_AREA_SIZE : f64= 0.5;
// const INIT_FINEBROT_TARGET_re : f64= 0.5;
// const INIT_FINEBROT_TARGET_im : f64= -0.38;
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;
const RESOLUTION_MULTIPLIER: ResolutionMultiplier = Single;
const REPEAT: bool = false;
const SAVE_IMAGES: bool = false;
// const PALETTE: Palette = palette_black_to_white_exp2();

struct FatStarTentacle {
    pub name: String,
}

impl Fractal<Mem> for FatStarTentacle {
    fn math(&self, m: &mut Mem, origin_re: f64, origin_im: f64) {
        m.square();
        m.conjugation();
        m.square();
        m.plus(origin_re, origin_im);
    }

    fn path_test(&self, min: u32, max: u32, length: u32, iterator: u32) -> bool {
        todo!()
    }
}

fn main() {
    // TODO
}


#[test]
fn test_math() {
    let fat_star_tentacle = FatStarTentacle { name: NAME.to_string() };
    let mut m = Mem { re: 0.0, im: 0.0 };
    fat_star_tentacle.math(&mut m, 1.0, 0.1);
    assert_eq!(m.re, 1.0);
    assert_eq!(m.im, 0.1);
}
