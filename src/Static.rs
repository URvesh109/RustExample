static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read only-memory";
        println!("Static string {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("Coerced_static: {}", coerced_static);
    }
    println!("Num is {}", NUM);
}
