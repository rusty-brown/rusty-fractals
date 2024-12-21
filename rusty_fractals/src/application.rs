use crate::area::Area;
use crate::data_image::DataImage;
use crate::fractal::CalculationType::StaticImage;
use crate::fractal::FractalType::MandelbrotType;
use crate::fractal::{FractalConfig, FractalMath};
use crate::machine;
use crate::machine::Machine;
use crate::mem::Mem;
use fltk::app::{event_button, event_coords, event_key, App};
use fltk::enums::{Color, Event, Key};
use fltk::window::DoubleWindow;
use fltk::{app, draw, prelude::*, window::Window};
use image::{Pixel, Rgb};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Application {
    // machine: Arc<Machine<'lt, F>>,

    /* DoubleWindow class provides a **double-buffered** window.
    - In double buffering:
    - All drawing operations are first performed in an **off-screen buffer**.
    - Once the drawing is complete, the off-screen buffer is copied (or "flipped") onto the screen in a single operation.
    - This eliminates flickering during redraws, as the user only sees the final, fully-drawn frame.*/
    pub window: Arc<Mutex<DoubleWindow>>, // Shared ownership of the GUI Window
    // pub width: i32,
    // pub height: i32,
    // pub name: &'static str,
}

fn init(config: &FractalConfig) -> Arc<Mutex<Application>> {
    let mut window = Window::default();

    let width = config.width_x as i32;
    let height = config.height_y as i32;
    let name = config.name.clone();

    window.set_label(name);
    window.set_size(width, height);

    window.set_color(Color::Red);
    window.show();

    Arc::new(Mutex::new(Application {
        window: Arc::new(Mutex::new(window)),
    }))
}

/**
 * start the application
 */
pub fn execute<F: FractalMath + 'static>(config: FractalConfig, fractal: F) {
    println!("application.execute()");

    // self.window.center_screen();
    let app = app::App::default();
    

    let application_arc = init(&config);
    // let mut application = application_arc.lock().unwrap();

    println!("show()");
    // application.init_window_actions();

    // Clone the Arc, increasing its reference count

    println!("calculation - new thread ");

    let task = move || {
        // clone arc, not application
        machine::init(&config, fractal).execute_calculation(application_arc.clone());
    };
    rayon::spawn_fifo(task);

    println!("run().unwrap()");
    // The last line of the program
    app.run().unwrap();

    println!("execute() end.");
}

/**
 * Use static calls to communicate between app and Machine
 */
impl Application {
    pub fn init_window_actions(&mut self) {
        println!("show()");

        // initialize window color, filled rectangle
        // draw::set_draw_color(Color::from_rgb(40, 180, 150));
        // draw::draw_rectf(0, 0, width, height);

        self.window
            .lock()
            .unwrap()
            .handle(move |_, event| match event {
                Event::KeyDown => {
                    let ek = event_key();
                    match ek {
                        Key::Escape => {
                            println!("exit");
                            // TODO self.app.quit(); // `self` is still directly referenced here
                        }
                        _ => {}
                    }
                    match ek.to_char().unwrap() {
                        'i' => {
                            println!("i");
                            true
                        }
                        's' => {
                            println!("s");
                            true
                        }
                        ' ' => {
                            println!("space bar");
                            // TODO probably not the right method
                            // TODO self.machine.zoom_in_recalculate_pixel_positions();
                            true
                        }
                        _ => false,
                    }
                }
                Event::Released => {
                    // mouse button click
                    let left = event_button() == 1;
                    if left {
                        let (x, y) = event_coords();
                        println!("c: {} {}", x, y);
                        //self.machine.move_target(x as usize, y as usize);
                        //self.machine.zoom_in_recalculate_pixel_positions();
                    }
                    false
                }
                _ => false,
            });
    }
    pub fn window_repaint(&self, color: Color) {
        println!("window_repaint()");
        
        let mut window = self.window.lock().unwrap();
        window.set_color(color);
        window.redraw();
        app::awake(); // Ensure the event loop processes the change immediately

        // println!("window_draw()");
        // self.window.draw(move |_| {
        //     println!("draw");
        //     // never use self in here
        //     // locking / unlocking app for draw is not necessary
        //     // redraw() can't be called from draw().
        //
        //     draw::draw_rect_fill(0, 0, 200, 250, Color::from_rgb(40, 180, 150));
        // });
        // println!("window_draw() end.");

        // for y in 0..height {
        //     for x in 0..width {
        //         let (value, state, _, _, colour_index_o) =
        //             machine.data_image.values_at(x as usize, y as usize);
        //         let colour: Rgb<u8>;
        //         if !is_finished_any(state) {
        //             colour = colour_for_state(state);
        //         } else {
        //             match colour_index_o {
        //                 Some(pixel_colour) => {
        //                     colour = pixel_colour;
        //                 }
        //                 None => {
        //                     let mut mv = max_value.lock().unwrap(); // Access `max_value` via the cloned Arc
        //                     if value > *mv {
        //                         *mv = value;
        //                     }
        //                     // make color (3x) brighter
        //                     let mut cv = ((value * 3) as f64 / *mv as f64) * 255.0;
        //                     if cv > 255.0 {
        //                         cv = 255.0;
        //                     }
        //                     let c = cv as u8;
        //                     colour = Rgb([c, c, c]);
        //                 }
        //             }
        //         }
        //         let r = colour.channels().get(0).unwrap();
        //         let g = colour.channels().get(1).unwrap();
        //         let b = colour.channels().get(2).unwrap();
        //         draw::set_draw_color(Color::from_rgb(*r, *g, *b));
        //         draw::draw_point(x, y);
        //     }
        // }
    }
}

/* --------------
 * static methods
 * ----------- */

pub fn paint_path(area: &Area, data: &DataImage) {
    let path = &data.show_path.lock().unwrap();

    for p in path.as_slice() {
        let (x, y) = area.point_to_pixel(p[0], p[1]);
        draw::set_draw_color(Color::from_rgb(255, 215, 0));
        draw::draw_point(x as i32, y as i32);
    }
    // rendering must be done from main thread
    app::awake();
    app::redraw();
}

/**
 * rendering must be done from main thread
 */
pub fn paint_image_calculation_progress(xy: &[u32; 2], data: &DataImage) {
    let chunk_size_x = data.width / 20;
    let chunk_size_y = data.height / 20;

    let xx = xy[0] as usize;
    let yy = xy[1] as usize;

    let x_from = chunk_size_x * xx;
    let x_to = chunk_size_x * (xx + 1);
    let y_from = chunk_size_y * yy;
    let y_to = chunk_size_y * (yy + 1);

    let lr = app::lock();

    for y in y_from..y_to {
        for x in x_from..x_to {
            let colour_index_o = data.colour_at(x, y);
            match colour_index_o {
                None => {
                    println!("paint_image_calculation_progress(): colour_index_o is None");
                }
                Some(ci) => {
                    draw_colored_point(x, y, &ci);
                }
            }
            // rendering must be done from main thread
            app::awake();
            app::redraw();
        }
    }
}

pub fn paint_image_result(data: &DataImage) {
    println!("paint_image_result()");
    for y in 0..data.height {
        for x in 0..data.width {
            let colour_index_o = data.colour_at(x, y);
            match colour_index_o {
                None => {
                    println!("paint_image_result(): colour_index_o is None");
                }
                Some(ci) => {
                    draw_colored_point(x, y, &ci);
                }
            }
        }
    }
    // rendering must be done from main thread
    app::awake();
    app::redraw();
}

fn draw_colored_point(x: usize, y: usize, color: &Rgb<u8>) {
    let r = *color.channels().get(0).unwrap();
    let g = *color.channels().get(1).unwrap();
    let b = *color.channels().get(2).unwrap();

    draw::set_draw_color(Color::from_rgb(r, g, b));
    draw::draw_point(x as i32, y as i32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_it() {}
}
