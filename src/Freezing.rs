fn main() {
    let mut _mutable_integer = 7;
    {
        let ref large_integer = _mutable_integer;
        // _mutable_integer = 50;
        println!("Immutabley borrowed integer {}", large_integer);
    }
    _mutable_integer = 50;
    println!("{}", _mutable_integer);
}
