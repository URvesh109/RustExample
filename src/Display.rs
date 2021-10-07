use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i32, i32);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minMax = MinMax(0, 4);
    println!("Compare Structure");
    println!("Display {}", minMax);
    println!("Debug {:?}", minMax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big}, small range is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 4.3 };
    println!("Compare Structure");
    println!("Display {}", point);
    println!("Debug {:?}", point);

    // Error
    // println!("Point 2d binary version {:b}?", point)
}
