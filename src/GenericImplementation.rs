struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 4.4 };
    let y = GenVal { gen_val: 'a' };
    let z = GenVal { gen_val: 3 };

    println!("{} {} {}", x.value(), y.value(), z.value());
}
