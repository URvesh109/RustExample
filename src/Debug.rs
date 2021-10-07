#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("{:?} months in a year ", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!("Struct is printable {:?}", Structure(2));
    println!("Now {:?} will print!", Deep(Structure(3)));

    let name = "Urvesh";
    let age = 30;
    let person = Person { name, age };
    println!("Person Details {:#?}", person);
}
