//Introducing Question
use std::num::ParseIntError;

type Aliasedresult<T> = Result<T, ParseIntError>;

fn multiply(first_number: &str, second_number: &str) -> Aliasedresult<i32> {
    // let first = first_number.parse::<i32>().unwrap();
    // let second = second_number.parse::<i32>().unwrap();
    let first = first_number.parse::<i32>()?;
    let second = second_number.parse::<i32>()?;
    Ok(first * second)
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
