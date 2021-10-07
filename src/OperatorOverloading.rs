use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

fn main() {
    println!("Foo + Bar {:?}", Foo + Bar);
    println!("Bar + Foo {:?}", Bar + Foo);
}
