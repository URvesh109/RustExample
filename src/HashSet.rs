//HashSet

use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let mut b: HashSet<i32> = vec![2, 3, 6, 7, 8, 3, 4, 4].into_iter().collect();

    println!("Union {:?}", a.union(&b));
    println!("Difference {:?}", a.difference(&b));
    println!("Intersection {:?}", a.intersection(&b));
    println!("Symmetric Diff {:?}", a.symmetric_difference(&b));
}
