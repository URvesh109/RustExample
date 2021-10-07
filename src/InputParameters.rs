//Fn: the closure captures by reference (&T)
//FnMut: the closure captures by mutable reference (&mut T)
//FnOnce: the closure captures by value (T)

fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let mut diary = || {
        println!("I said {}", greeting);
        farewell.push_str(" !!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep");
        // mem::drop(farewell);
    };

    apply(diary);

    let double = |x| x * 2;
    println!(" 3 doubled {}", apply_to_3(double));
}
