use crate::application::Application;
use crate::machine_mandelbrot;
use fltk::app::{event_key, App};
use fltk::enums::ColorDepth::Rgb8;
use fltk::enums::{Event, Key};
use fltk::image::RgbImage;
use fltk::{frame::Frame, prelude::*, window::Window};
use image::{ImageBuffer, Rgb};
use rusty_fractals_common::area::Area;
use rusty_fractals_common::data_image::{colour_for_state, DataImage};
use rusty_fractals_common::fractal::{FractalCommon, FractalMandelbrotCommon, FractalMath};
use rusty_fractals_common::mem::Mem;
use rusty_fractals_common::palette::Palette;
use rusty_fractals_common::pixel_states::DomainElementState;
use rusty_fractals_common::pixel_states::DomainElementState::{
    ActiveNew, FinishedSuccess, FinishedSuccessPast, FinishedTooLong, FinishedTooShort,
    HibernatedDeepBlack,
};
use rusty_fractals_common::{fractal};

const INT: i32 = 100;

pub fn show_state_colours() {
    let width = 600;
    let height = 100;
    let mut image = image::RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        color_interval(&mut image, 0, 1, y, FinishedSuccessPast);
        color_interval(&mut image, 1, 2, y, FinishedSuccess);
        color_interval(&mut image, 2, 3, y, ActiveNew);
        color_interval(&mut image, 3, 4, y, FinishedTooShort);
        color_interval(&mut image, 4, 5, y, FinishedTooLong);
        color_interval(&mut image, 5, 6, y, HibernatedDeepBlack);
    }
    pop_app_window(width, height, image);
}

fn pop_app_window(width: i32, height: i32, image: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let app = App::default();
    let image_rgb = RgbImage::new(image.as_raw(), width, height, Rgb8).unwrap();
    let mut window = Window::default()
        .with_label("test window")
        .with_size(width, height)
        .center_screen();
    let mut frame = Frame::new(0, 0, width, height, "");
    frame.set_image(Some(image_rgb));
    window.add(&frame);
    window.handle(move |_, event| match event {
        Event::KeyDown => {
            let ek = event_key();
            if ek == Key::Escape {
                app.quit();
                return true;
            }
            false
        }
        _ => false,
    });
    window.end();
    window.show();
    app.run().unwrap();
}

fn color_interval(
    image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    from: i32,
    to: i32,
    y: i32,
    state: DomainElementState,
) {
    for x in (from * INT)..(to * INT) {
        image.put_pixel(x as u32, y as u32, colour_for_state(state));
    }
}

/**
 * A fractal for any test
 */
pub struct FractalTest<'lt> {
    pub app: Application<'lt>,
}

impl FractalMath<Mem> for FractalTest<'_> {
    fn math(&self, mc: &mut Mem, origin_re: f64, origin_im: f64) {
        mc.square();
        mc.plus(origin_re, origin_im);
    }
}

impl<'lt> FractalMandelbrotCommon<'lt> for FractalTest<'lt> {
    fn calculate_path(&self, iteration_max: u32, origin_re: f64, origin_im: f64) -> (u32, f64) {
        fractal::calculate_mandelbrot_path(self, iteration_max, origin_re, origin_im)
    }
    fn calculate_mandelbrot(&self) {
        let fm = machine_mandelbrot::init();
        fm.calculate_mandelbrot(self);
    }
    fn palette_zero(&self) -> &Palette {
        &self.palette_zero()
    }
}

impl<'lt> FractalCommon<'lt> for FractalTest<'lt> {
    fn name(&self) -> &'static str {
        "Test Fractal"
    }
    fn update(&self) {}
    fn zoom_in(&self) {}
    fn width(&self) -> usize {
        10
    }
    fn height(&self) -> usize {
        10
    }
    fn data_image(&self) -> &DataImage<'lt> {
        &self.app.data_image
    }
    fn palette(&self) -> &Palette<'lt> {
        &self.app.palette
    }
    fn min(&self) -> u32 {
        0
    }
    fn max(&self) -> u32 {
        10
    }
    fn area(&self) -> &Area<'lt> {
        &self.app.area
    }
    fn recalculate_pixels_positions_for_next_calculation(&self, is_mandelbrot: bool) {
        println!("recalculate_pixels_positions_for_next_calculation is_mandelbrot: {}", is_mandelbrot);
        // self.app.recalculate_pixels_positions_for_next_calculation(is_mandelbrot);;
    }
    fn move_target(&self, x: usize, y: usize) {
        println!("move_target x: {}, y: {}", x, y);
    }
    fn zoom_and_recalculate(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
