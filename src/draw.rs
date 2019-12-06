use crate::display::{Pixel, Color, Display};

pub fn set_pixel(pixel: Pixel, color: Color, display: &mut Display) {
    if pixel.x >= display.size_x {
        panic!("Pixel out of bounds ({},{})",pixel.x,pixel.y);
    }
    if pixel.y >= display.size_y {
        panic!("Pixel out of bounds ({},{})",pixel.x,pixel.y);
    }
    display.disp[pixel.y as usize][pixel.x as usize] = color;
}

pub fn plot_line(p0: Pixel, p1: Pixel, color:Color, display: &mut Display) {
    if (p1.y - p0.y).abs() < (p1.x - p0.x).abs() {
        if p0.x > p1.x {
            plot_line_low(p1, p0, color, display);
        }
        else {
            plot_line_low(p0, p1, color, display);
        }
    }
    else {
        if p0.y > p1.y {
            plot_line_high(p1, p0, color, display);
        }
        else {
            plot_line_high(p0, p1, color, display);
        }
    }
}

fn plot_line_low(p0: Pixel, p1: Pixel, color:Color, display: &mut Display) {
    let dx = p1.x - p0.x;
    let mut dy = p1.y - p0.y;
    let mut yi = 1;
    if dy < 0 {
        dy = -dy;
        yi = -1;
    }
    let mut d = 2*dy - dx;
    let mut y = p0.y;

    for x in p0.x..p1.x {
        set_pixel(Pixel::new(x,y), color, display);
        if d > 0 {
            y = y + yi;
            d = d - 2*dx;
        }
        d = d + 2*dy;
    }
}

fn plot_line_high(p0: Pixel, p1: Pixel, color:Color, display: &mut Display) {
    let mut dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    let mut xi = 1;
    if dx < 0 {
        dx = -dx;
        xi = -1;
    }
    let mut d = 2*dx - dy;
    let mut x = p0.x;

    //println!("{:?},{:?}",p0,p1);
    for y in p0.y..p1.y {
        //println!("{},{}",x,y);
        set_pixel(Pixel::new(x,y), color, display);
        if d > 0 {
            x = x + xi;
            d = d - 2*dy;
        }
        d = d + 2*dx;
    }
}
