use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lan_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:3}':{} {:3} {}",
            self.name,
            self.lat.abs(),
            lan_c,
            self.lon.abs(),
            lon_c
        )
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 54.43773,
            lon: -6.7872,
        },
        City {
            name: "Oslo",
            lat: 59.77,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.35,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{:?}", color);
    }
}
