use std::mem;

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 1.1 })
}

fn main() {
    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let box_rectangle = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    let box_point = Box::new(origin());

    //Double Indirection
    let box_in_a_box = Box::new(boxed_origin());

    println!("Bytes Point occupies {}", mem::size_of_val(&point));
    println!("Bytes Rectangle occupies {}", mem::size_of_val(&rectangle));
    println!(
        "Bytes Box_rect occupies {}",
        mem::size_of_val(&box_rectangle)
    );
    println!(
        "Bytes Boxed box occupes {}",
        mem::size_of_val(&box_in_a_box)
    );
    let unboxed_point = *box_point;
    println!("Uboxed occupies {}", mem::size_of_val(&unboxed_point));
}
