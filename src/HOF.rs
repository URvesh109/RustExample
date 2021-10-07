fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("Find sum of all the squared numbers under 1000");
    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= 1000 {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("Imperative Programming {}", acc);

    let sum_of_squared_odd_number: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // Below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // That are odd
        .fold(0, |acc, n_squared| acc + n_squared); // Sum them

    println!("Functional style: {}", sum_of_squared_odd_number);
}
