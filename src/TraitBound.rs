trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.length
    }
}

// trait bound
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rec = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    println!("{}", area(&rec));
}
