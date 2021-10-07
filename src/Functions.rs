#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
}

impl Rectangle {
    fn new(x: i32, y: i32) -> Rectangle {
        Rectangle { x, y }
    }

    fn myRefSelf(&self) {
        println!("Ref Self is {:?}", self);
    }

    fn myMutRefSelf(&mut self) {
        println!("Mut ref self {:?}", self);
    }

    fn destroy(self) {
        println!("Self is {:?}", self);
    }
}

fn main() {
    let rect = Rectangle::new(23, 23);

    rect.myRefSelf();
    rect.destroy();
}
