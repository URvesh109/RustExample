// Aliased of Result
use std::num::ParseIntError;

type Aliasedresult<T> = Result<T, ParseIntError>;

fn multiply(first_number: &str, second_number: &str) -> Aliasedresult<i32> {
    first_number
        .parse::<i32>()
        .and_then(|first| second_number.parse::<i32>().map(|second| first * second))
}

fn show_result(result: Aliasedresult<i32>) {
    match result {
        Ok(v) => println!("Value is {}", v),
        Err(e) => println!("Error {}", e),
    }
}

fn main() {
    let result = multiply("22", "2");
    show_result(result);
    let fail = multiply("t", "2");
    show_result(fail);
}
