fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1, 2, 3, 4];
    println!("Initial Vector {:?}", xs);

    // collected_iterator.push(1);
    println!("Vector length: {}", xs.len());
    println!("Indexed {}", xs[2]);
    println!("Poped {:?}", xs.pop());
    // println!("Fourth element {}", xs[4]);

    println!("Contents of xs");
    for x in xs.iter() {
        println!("> {}", x);
    }

    //enumerate
    for (i, x) in xs.iter().enumerate() {
        println!("I is {} and x is {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }

    println!("updated vector {:?}", xs);
}
