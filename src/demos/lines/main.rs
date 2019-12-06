use gfxlib::display::{Pixel, Color, init_display, write_display};
use gfxlib::draw::{set_pixel, plot_line};
use std::f64::consts::{PI};

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const R: f64 = 200.;
const NUM_POINTS: f64 = 21.;

fn main() {
    let mut points: Vec<(i32,i32)> = Vec::new();

    for i in 0..(NUM_POINTS as i32) {
        let i = i as f64;
        points.append(&mut vec![((R*(i*2.0*PI/NUM_POINTS).cos()) as i32,
                                 (R*(i*2.0*PI/NUM_POINTS).sin()) as i32)]);
    }

    points = points.iter().map(|(x, y)| (WIDTH/2 + x, HEIGHT/2 + y)).collect();
    println!("{:#?}", points);
    let mut disp = init_display(WIDTH, HEIGHT, Color::new(0,0,0));
    let mut h:f64 = 0.;
    for (x0, y0) in &points {
        h = (h + (360. / NUM_POINTS)) % 360.;
        for (x1, y1) in &points {
            plot_line(Pixel::new(*x0,*y0), Pixel::new(*x1,*y1), Color::new_hsv(h, 1., 1.), &mut disp);
        }
    }

    write_display("img/lines.ppm", &disp)
}
