fn foo() -> ! {
    panic!("THis is paniced function");
}

fn some_fn() {
    println!("This empty tuple");
}

fn main2() {
    let a = foo();
    println!("Can't see you");
}

fn main1() {
    let a = some_fn();
    println!(" I can see you");
}

fn sum_odd_number(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        let addition: u32 = match i % 2 == 1 {
            true => i,
            false => continue,
        };
        println!("{}", addition);
        acc += addition;
    }
    acc
}

fn main() {
    println!("Sum {}", sum_odd_number(9));
}
