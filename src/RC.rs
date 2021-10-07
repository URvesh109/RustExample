use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("---- rc a is created-----");
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("---- rc_b is created ---");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference Count rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count rc_b: {}", Rc::strong_count(&rc_a));
            //Two Rc's are equal if their inner value are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            //we can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            println!("--- rc_b is dropped out of scope --- ");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }
}
