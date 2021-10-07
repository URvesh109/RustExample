struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut sequence = 0..3;

    println!("Four consecutive next call 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using for");
    for i in 0..5 {
        println!("> {}", i);
    }

    println!("The next four items take");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four items skip and take");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let arr = [1, 2, 3, 4];
    for i in arr.iter() {
        println!("{}", i);
    }
}
