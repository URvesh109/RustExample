#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

type TempPoint = Point;

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    point.y = 5;
    let borrowed_point = &mut point;
    let another_point = &mut point;
    let another_point1 = &mut point;
    let another_point2 = &mut point;
    println!("{}", another_point2.x);

    let k = TempPoint {
        x: 12,
        y: 12,
        z: 12,
    };

    // println!("{} {} {}", borrowed_point.x, another_point.x, point.x);
    // let mutable_point = &mut point;
    // println!("{}", borrowed_point.y);
}
