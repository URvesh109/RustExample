struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 1;
    let number_2 = 2;
    let container = Container(number_1, number_2);
    println!(
        "Does container contain {} and {} : {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First {}", container.first());
    println!("Last {}", container.last());
    println!("Difference {}", difference(&container));
}
