fn main() {
    print!("{}", 32);
    println!("{}", 32);
    // Positional arguments
    println!("{0}, this is {1}, this is {0}", "ALice", "Bob");

    //As can be name argument
    println!("{lang} is my favourite language", lang = "Rust");

    //Special formatting can be specified after a ":"
    println!(
        "{} of {:b} people know the binary, the other half doesn't know",
        1, 16
    );

    //You can right align text with specific width
    println!("{number:>width$}", number = 1, width = 6);
}
