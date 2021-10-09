//Boxing erros preserving original error

use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {
    // fn description(&self) -> &str {
    //     "invalid"
    // }
    // fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
    //     None
    // }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn double_first1(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into()) // Converts to Box
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) //Converts to Box
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("Value is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // let numbers = vec!["11", "22", "33"];
    // let empty = vec![];
    // let strings = vec!["1.1", "33"];

    // print(double_first(numbers));
    // print(double_first(empty));
    // print(double_first(strings));
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));
    assert_eq!(x.ok_or(0), Ok("foo"));
}
