struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &str;
}

impl Animal for Sheep {
    fn noise(&self) -> &str {
        "baaaah?"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &str {
        "maaaaa"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 5.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let num = 3.4;
    let animal = random_animal(num);
    println!("{}", animal.noise());
}
