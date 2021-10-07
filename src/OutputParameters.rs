//Beyond this, the move keyword must be used, which signals that all captures occur by value.
//This is required because any captures by reference would be dropped as soon as the function exited, leaving invalid references in the closure.

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnMut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_FnOnce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plan = create_fn();
    let mut fn_mut = create_fnMut();
    let fn_once = create_FnOnce();

    fn_plan();
    fn_mut();
    fn_once();
}
