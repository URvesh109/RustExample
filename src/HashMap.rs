//HashMap

use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. Please hang up and try again"
        }
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is fred",
        _ => "Hi, who is this again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "789-1364");
    contacts.insert("Daniel", "7891-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Rober", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", number),
        _ => println!("Don't have Daniel's number"),
    }

    contacts.insert(&"Daniel", "167-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashlyey: {}", number),
        _ => println!("Don't have Ashley's number."),
    }
    println!();
    contacts.remove(&"Ashley");

    for (contacts, &number) in contacts.iter() {
        println!("Calling {} {}", contacts, number);
    }
    println!();
}
