//Centimeters tuples can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(f64);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 * 2.54)
    }
}

fn main() {
    let foot = Inches(12.0);
    let meters = Centimeters(100.0);
    let cmp = {
        if foot.to_centimeters() < meters {
            "smaller"
        } else {
            "bigger"
        }
    };
    println!("{}", cmp);
}
