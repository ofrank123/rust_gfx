extern crate num;
use num::complex::Complex;

use gfxlib::display::{Display, Color, Pixel, init_display, set_pixel, write_display};

const MAX_ITER: i32 = 80;
const RE_START: f64 = -2.0;
const RE_END: f64 = 1.0;
const IM_START: f64 = -1.0;
const IM_END: f64 = 1.0;
const WIDTH: usize = 600 * 2;
const HEIGHT: usize = 400 * 2;

fn main() {
    let mut disp = init_display(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let c = Complex::new(RE_START + (x as f64 / WIDTH as f64) * (RE_END - RE_START),
                                 IM_START + (y as f64 / HEIGHT as f64) * (IM_END - IM_START));
            let m = mandelbrot(c);
            let hue = (360. * m as f64 / MAX_ITER as f64).floor();
            let sat = 1.;
            let val = if m < MAX_ITER {1.} else {0.};
            set_pixel(Pixel::new(x, y), Color::new_hsv(hue,sat,val), &mut disp);
        }
    }
    write_display("img/test.ppm", &disp);
}

fn mandelbrot(c: Complex<f64>) -> i32 {
    let mut z = Complex::from(0.0);
    let mut n = 0;
    while abs(z) <= 2.0 && n < MAX_ITER {
        z = z*z + c;
        n += 1;
    }
    n
}

fn abs(c: Complex<f64>) -> f64 {
    (c.re*c.re + c.im*c.im).sqrt()
}
