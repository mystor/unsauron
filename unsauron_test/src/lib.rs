#[macro_use]
extern crate unsauron;

#[test]
fn it_works() {
    let a = 5;
    let b = 5;
    let c = 5;
    let d = 5;
    println!("{}", unsauron!(a + b * c + d));
}