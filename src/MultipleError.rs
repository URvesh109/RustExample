// Multiple error types
use std::num::ParseIntError;

fn double_first1(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    // opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let numbers = vec!["3", "4", "5"];
    let empty = vec![];
    let strings = vec!["tofu", "33", "66"];
    println!("First doubled {:?}", double_first(numbers));
    println!("First doubled {:?}", double_first(empty));
    println!("First doubled {:?}", double_first(strings));
}
