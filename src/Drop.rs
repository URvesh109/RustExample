struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    fn drop(&mut self) {
        println!("Drop is called {}", self.name)
    }
}

fn main() {
    let mut _a = Dropable { name: "a" };
    {
        //block a
        let _b = Dropable { name: "b" };
    }
    println!("Checking {}", _a.name);
}
