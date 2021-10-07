//Map and Then for Result

use std::num::ParseIntError;

fn multiply(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
    match f_n_str.parse::<i32>() {
        Ok(first_number) => match s_n_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
fn multiply_v2(f_n_str: &str, s_n_str: &str) -> Result<i32, ParseIntError> {
    f_n_str.parse::<i32>().and_then(|first_number| {
        s_n_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("N is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let result = multiply("33", "33");
    print(result);
    let result1 = multiply("tt", "33");
    print(result1);
    let result_v2 = multiply_v2("10", "2");
    let result_v2_1 = multiply_v2("tt", "22");
    print(result_v2);
    print(result_v2_1);
}
