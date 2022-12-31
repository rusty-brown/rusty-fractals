// The method used for perfect coloring is
// - Gather all screen pixels and order them by value
// - Count how many pixels should be colored by each color from spectrum
// - Zero elements and noise color by the lowest color
// - Color all significant pixels ordered by value

use std::cmp::Ordering::Equal;
use rgb::RGB;
use constants::COLORING_THRESHOLD;
use rusty_fractals_common::constants;
use crate::palette::Palette;
use crate::result_pixels::ResultPixels;

// for Nebula like fractals
struct Pix {
    x: usize,
    y: usize,
    value: u32,
}

// for Mandelbrot like fractals
struct Mix {
    x: usize,
    y: usize,
    value: u32,
    quad: f64,
    quid: f64,
}

pub fn perfectly_color_values(mut result_pixels: &ResultPixels, palette: Palette) -> RgbImage<RGB<u8>> {
    let width = result_pixels.width;
    let height = result_pixels.height;

    // Result pixels, order by value
    let mut pixels: Vec<Pix> = Vec::new();

    let mut zero_value_elements = 0;

    // read screen values
    for y in 0..height {
        for x in 0..width {
            let v = result_pixels.value_at(x, y);
            if v <= COLORING_THRESHOLD {
                zero_value_elements += 1;
            }
            pixels.push(Pix { x, y, value: v });
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|first, second| first.1.cmp(&second.1));

    let all_pixels_total = width * height;
    let all_pixels_non_zero = all_pixels_total - zero_value_elements;
    let palette_color_count = palette.spectrum.len();
    let single_color_use = all_pixels_non_zero as f64 / palette_color_count as f64;
    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements + left + (single_color_use * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero);
    log.debug("Spectrum, available colors: " + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use);
    log.debug("left:                       " + left);
    log.debug("------------------------------------");

    let result_image: RgbImage = RgbImage::new(width, height);

    // paint mismatched pixel amount with the least value colour
    for pi in 0..(left + zeroValueElements) {
        let sp = pixels.get(pi);
        result_image.put_pixel(sp.x, sp.y, palette.spectrum_value(0));
    }

    // color all remaining pixels, these are order by value
    for palette_colour_index in 0..palette_color_count {
        for _ in 0..single_color_use {
            // color all these pixels with same color
            let sp = pixels.get(pi += 1);
            if sp.value() <= COLORING_THRESHOLD {
                // color zero-value elements and low-value-noise with the darkest color
                result_image.put_pixel(sp.x, sp.y, palette.spectrum_value(0));
            } else {
                // perfect-color all significant pixels
                result_image.put_pixel(sp.x, sp.y, palette.spectrum_value(palette_colour_index));
            }
        }
    }
    log.debug("painted:                   " + pi);

    // Behold, the coloring is perfect

    log.debug("clear pixels");
    pixels.clear();

    result_image
}


fn perfectly_color_values_euler() -> ImageBuffer<RGB<u8>> {
    let width = result_pixels.width;
    let height = result_pixels.height;

    // Result pixels, order by value
    let mut pixels_red: Vec<Pix> = Vec::new();
    let mut pixels_green: Vec<Pix> = Vec::new();
    let mut pixels_blue: Vec<Pix> = Vec::new();

    let mut zero_value_elements_red = 0;
    let mut zero_value_elements_green = 0;
    let mut zero_value_elements_blue = 0;

    // identify zero and low-value elements as zero or noise
    let threshold = 1;

    // read screen values
    for y in 0..height {
        for x in 0..width {
            let r = result_pixels.value_at(x, y, red);
            let g = result_pixels.value_at(x, y, green);
            let b = result_pixels.value_at(x, y, blue);
            if r <= threshold {
                zero_value_elements_red += 1;
            }
            if g <= threshold {
                zero_value_elements_green += 1;
            }
            if b <= threshold {
                zero_value_elements_blue += 1;
            }
            pixels_red.push(Pix { x, y, value: r });
            pixels_green.push(Pix { x, y, value: g });
            pixels_blue.push(Pix { x, y, value: b });
        }
    }

    // order pixels from the smallest to the highest value
    pixels_red.sort_by(|first, second| first.1.cmp(&second.1));
    pixels_green.sort_by(|first, second| first.1.cmp(&second.1));
    pixels_blue.sort_by(|first, second| first.1.cmp(&second.1));

    let all_pixels_total = width * height;
    let all_pixels_non_zero_red = all_pixels_total - zero_value_elements_red;
    let all_pixels_non_zero_green = all_pixels_total - zero_value_elements_green;
    let all_pixels_non_zero_blue = all_pixels_total - zero_value_elements_blue;
    let palette_color_count = PaletteEuler3.colorResolution();
    let single_color_use_red = all_pixels_non_zero_red / palette_color_count;
    let single_color_use_green = all_pixels_non_zero_green / palette_color_count;
    let single_color_use_blue = all_pixels_non_zero_blue / palette_color_count;
    let left_red = all_pixels_non_zero_red - (palette_color_count * single_color_use_red);
    let left_green = all_pixels_non_zero_green - (palette_color_count * single_color_use_green);
    let left_blue = all_pixels_non_zero_blue - (palette_color_count * single_color_use_blue);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements_red + left_red + (single_color_use_red * palette_color_count)));
    log.debug("--------------------------->" + (zero_value_elements_green + left_green + (single_color_use_green * palette_color_count)));
    log.debug("--------------------------->" + (zero_value_elements_blue + left_blue + (single_color_use_blue * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements_red);
    log.debug("Zero value pixels to paint: " + zero_value_elements_green);
    log.debug("Zero value pixels to paint: " + zero_value_elements_blue);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_red);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_green);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero_blue);
    log.debug("Spectrum, available colors: " + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use_red);
    log.debug("Pixels per each color:      " + single_color_use_green);
    log.debug("Pixels per each color:      " + single_color_use_blue);
    log.debug("left:                       " + left_red);
    log.debug("left:                       " + left_green);
    log.debug("left:                       " + left_blue);
    log.debug("------------------------------------");

    // paint mismatched pixel amount with the least value colour
    for pi_red in 0..(leftRed + zeroValueElementsRed) {
        let sp = pixelsRed.get(pi_red);
        PixelsEulerFinebrot.set(sp.x, sp.y, red, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_colour_index in 0..palette_color_count {
        for _ in 0..single_color_use_red {
            // color all these pixels with same color
            let sp = pixels_red.get(pi_red += 1);
            if sp.pixelValue() <= threshold {
                PixelsEulerFinebrot.set(sp.x, sp.y, red, 0);
            } else {
                // perfect-color all significant pixels
                PixelsEulerFinebrot.set(sp.x, sp.y, red, PaletteEuler3.getSpectrumValueRed(palette_colour_index).getRed());
            }
        }
    }

    for pi_green in 0..(leftGreen + zeroValueElementsGreen) {
        sp = pixelsGreen.get(pi_green);
        PixelsEulerFinebrot.set(sp.x, sp.y, green, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_colour_index in 0..palette_color_count {
        for _ in 0..single_color_use_green {
            // color all these pixels with same color
            sp = pixelsGreen.get(pi_green += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                PixelsEulerFinebrot.set(sp.x, sp.y, green, 0);
            } else {
                // perfect-color all significant pixels
                PixelsEulerFinebrot.set(sp.x, sp.y, green, PaletteEuler3.getSpectrumValueGreen(palette_colour_index).getGreen());
            }
        }
    }

    for pi_blue in 0..(leftBlue + zeroValueElementsBlue) {
        sp = pixelsBlue.get(pi_blue);
        PixelsEulerFinebrot.set(sp.x, sp.y, blue, 0);
    }
    // color all remaining pixels, these are order by value
    for palette_colour_index in 0..palette_color_count {
        for _ in 0..single_color_use_blue {
            // color all these pixels with same color
            sp = pixelsBlue.get(pi_blue += 1);
            if sp.pixelValue() <= threshold {
                // color zero-value elements and low-value-noise with the darkest color
                PixelsEulerFinebrot.set(sp.x, sp.y, blue, 0);
            } else {
                // perfect-color all significant pixels
                PixelsEulerFinebrot.set(sp.x, sp.y, blue, PaletteEuler3.getSpectrumValueBlue(palette_colour_index).getBlue());
            }
        }
    }

    // read 3 euler spectra colors and write image colors
    for y in 0..height {
        for x in 0..width {
            let r = result_pixels.value_at(x, y, red);
            let g = result_pixels.value_at(x, y, green);
            let b = result_pixels.value_at(x, y, blue);
            FinebrotImage.setRGB(x, y, new Color(r, g, b).getRGB());
        }
    }

    // Behold, the coloring is perfect

    log.debug("clear pixels");
    pixelsRed.clear();
    pixelsGreen.clear();
    pixelsBlue.clear();
    PixelsEulerFinebrot.clear();

    result_image
}

const NEIGHBOR_COORDINATES: [[i8; 2]; 8] = [[-1, -1], [0, -1], [1, -1], [-1, 0], [1, 0], [-1, 1], [0, 1], [1, 1]];

fn perfectly_color_values_mandelbrot() -> ImageBuffer<RGB<u8>> {
    log.debug("perfectly_color_values()");

    let width = result_pixels.width;
    let height = result_pixels.height;

    // Result pixels, order by value
    let mut pixels: Vec<Mix> = Vec::new();
    let mut pixels_zero: Vec<Mix> = Vec::new();

    let mut zero_value_elements = 0;

    // read screen values

    for y in 0..height {
        for x in 0..width {
            let el = PixelsMandelbrot.elAt(x, y);
            if el.value == 0 {
                zero_value_elements += 1;
                pixels_zero.add(MandelbrotPixelFactory.make(el, x, y));
            } else {
                MandelbrotPixel
                mp = MandelbrotPixelFactory.make(el, x, y);
                pixels.add(mp);
                field[x][y] = mp;
            }
        }
    }

    //  order pixels from the smallest to the highest value
    pixels.sort_by(|first, second| {
        let c = first.value.cmp(&second.value);
        if c == Equal {
            first.quid.cmp(&second.quid)
        }
        c
    });
    pixels_zero.sort_by(|first, second| first.quad.cmp(&second.quad));
    
    let all_pixels_total = width * height;
    let all_pixels_non_zero = all_pixels_total - zero_value_elements;
    let palette_color_count = Palette.colorResolution();
    let single_color_use = all_pixels_non_zero / palette_color_count;

    let left = all_pixels_non_zero - (palette_color_count * single_color_use);

    log.debug("------------------------------------");
    log.debug("All pixels to paint:        " + all_pixels_total);
    log.debug("--------------------------->" + (zero_value_elements + left + (single_color_use * palette_color_count)));
    log.debug("Zero value pixels to paint: " + zero_value_elements);
    log.debug("Non zero pixels to paint:   " + all_pixels_non_zero);
    log.debug("Spectrum, available colors:>" + palette_color_count);
    log.debug("Pixels per each color:      " + single_color_use);
    log.debug("left:                       " + left);
    log.debug("------------------------------------");

    // paint mismatched pixel amount with the least but not the lowest value colour
    while pi < left {
        let mp = pixels.get(pi + +);
        MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(0).getRGB());
    }

    let mut palette_colour_index = 0;
    while palette_colour_index < palette_color_count {
        for _ in 0..singleColorUse {
            let mp = pixels.get(pi + +);
            mp.colorValue(palette_colour_index);
            MandelbrotImage.setRGB(mp.px, mp.py, Palette.getSpectrumValue(palette_colour_index).getRGB());
        }
        palette_colour_index += 1;
    }

    Assert.assertEquals(pixels.size(), pi);

    // Fix black dots caused by quad inverse imperfection
    // Keep incorrect quad results

    for mpp in pixels {
        let average_colour_index = ac_if_black_dot(mpp);
        if average_colour_index != -1 {
            let mpp.colorValue(average_colour_index);
            MandelbrotImage.setRGB(mpp.x, mpp.y, Palette.getSpectrumValue(average_colour_index).getRGB());
        }
    }

    // Paint insides of Mandelbrot set

    let zero_palette_color_count = PaletteZero.colorResolution();
    let zero_single_color_use = ((int)((double) zero_value_elements / (double) zero_palette_color_count));
    let zero_left = zero_value_elements - (zero_palette_color_count * zero_single_color_use);

    log.info("zero_palette_color_count:    > " + zero_palette_color_count);
    log.info("zero_single_color_use:       > " + zero_single_color_use);
    log.info("zero_left:                 > " + zero_left);

    let piz;
    for piz in 0..zeroLeft {
        mp = pixels_zero.get(piz);
        MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(0).getRGB());
    }
    for zeropalette_colour_index in 0..zeropalette_color_count {
        for _ in 0..zeroSingleColorUse {
            // color all these pixels with same color
            mp = pixels_zero.get(piz + +);
            MandelbrotImage.setRGB(mp.px, mp.py, PaletteZero.getSpectrumValue(zeropalette_colour_index).getRGB());
        }
    }

    log.debug("painted:                   " + pi);

    // Behold, the coloring is perfect

    log.debug("clear pixels");
    pixels.clear();
    pixels_zero.clear();

    result_image
}

// Return average color of neighbour elements
fn ac_if_black_dot(MandelbrotPixel mp) -> i32 {
    let pv = mp.pixelValue;
    let sum = 0;
    let neighbours = 0;
    for c in NEIGHBOR_COORDINATES {
        let a = mp.px + c[0];
        let b = mp.py + c[1];
        let n = check_domain(a, b);
        if n != null {
            if Math.abs(pv - n.pixelValue) > 2 {
                // verify only one value difference gradient //
                return -1;
            }
            sum += n.colorValue;
            neighbours += 1;
        } else {
            // don't fix elements of edges 
            -1
        }
    }

    let cv = mp.colorValue;
    let average_value = (int)(sum / neighbours);

    if cv < average_value - 5 {
        // darker 
        average_value
    }
    -1
}