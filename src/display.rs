use std::fs;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
}

impl Pixel {
    pub fn new(x: i32, y: i32) -> Pixel {
        Pixel {x,y}
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {red, green, blue}
    }

    // Gets new RGB color from the given HSV color
    pub fn new_hsv(hue: f64, sat: f64, val: f64) -> Color {
        let r;
        let g;
        let b;
        let c = val * sat;
        let x = c * (1.0 - (((hue / 60.) % 2.) - 1.).abs());
        let m = val - c;
        if hue >= 0.0 && hue < 60.0 {
            r = c + m;
            g = x + m;
            b = m;
        }
        else if hue >= 60.0 && hue < 120.0 {
            r = x + m;
            g = c + m;
            b = m;
        }
        else if hue >= 120.0 && hue < 180.0 {
            r = m;
            g = c + m;
            b = x + m;
        }
        else if hue >= 180.0 && hue < 240.0 {
            r = m;
            g = x + m;
            b = c + m;
        }
        else if hue >= 240.0 && hue < 300.0 {
            r = x + m;
            g = m;
            b = c + m;
        }
        else if hue >= 300.0 && hue < 360.0 {
            r = c + m;
            g = m;
            b = x + m;
        }
        else {
            r = 0.;
            g = 0.;
            b = 0.;
        }

        Color::new((r * 255.) as u8,(g * 255.) as u8,(b * 255.) as u8)
    }
}

pub struct Display {
    pub size_x: i32,
    pub size_y: i32,
    pub aspect: f64,
    pub disp: Vec<Vec<Color>>,
}

pub fn init_display(size_x: i32, size_y: i32, bg_color: Color) -> Display {
    let disp = vec![vec![bg_color; size_x as usize]; size_y as usize];
    Display {
        size_x,
        size_y,
        aspect: (size_x as f64) / (size_y as f64),
        disp,
    }
}

pub fn write_display(file_name: &str, display: &Display)  {
    let mut file_string = String::from("P3\n");
    file_string.push_str(&display.size_x.to_string());
    file_string.push_str(" ");
    file_string.push_str(&display.size_y.to_string());
    file_string.push_str("\n255\n");
    for arr in display.disp.iter() {
        for p in arr {
            file_string.push_str(&p.red.to_string());
            file_string.push_str(" ");
            file_string.push_str(&p.green.to_string());
            file_string.push_str(" ");
            file_string.push_str(&p.blue.to_string());
            file_string.push_str("\n");
        }
    }
    fs::write(file_name, file_string)
        .expect("Cannot write to file");
}
