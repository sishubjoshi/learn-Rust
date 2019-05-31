// use std::mem;

fn scope_and_shadowing() {

    let a =123;
    // let a = 756;    // OH MY RUST
    {
        let a = 777;
        let b = 456;
        println!("inside a,b = {},{}", a,b);
    }
    println!("a is {}", a);
}
fn main() {
    scope_and_shadowing()
}