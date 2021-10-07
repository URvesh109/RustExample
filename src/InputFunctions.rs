fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am function");
}

fn main() {
    let closuer = || println!("I am closure");

    call_me(closuer);
    call_me(function);
}
