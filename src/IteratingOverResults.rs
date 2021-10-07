// fn main() {
//     // let strings = vec!["11q", "93", "18"];
//     // let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
//     // println!("Results {:?}", numbers);

//     let strings1 = vec!["11q", "93", "18"];
//     let numbers: Vec<_> = strings1
//         .into_iter()
//         .map(|s| s.parse::<i32>())
//         .filter_map(Result::Ok)
//         .collect();
//     println!("Results {:?}", numbers);
// }

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
