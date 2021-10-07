#[derive(Debug)]
struct A;

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let _t = SingleGen(A);
    let _i32 = SingleGen(3);
    let _char = SingleGen('a');
    println!("{:?}", _t.0);
    println!("{}", _i32.0);
    println!("{}", _char.0);
    if _i32.0 == 3 {
        println!("It is equal");
    }
}
