#[macro_use]
extern crate unsauron;

use std::ops::Add;

#[test]
fn it_works() {
    let a = 5;
    let b = 5;
    let c = 5;
    let d = 5;
    // Expands to &(&(a) + &(&(b) + &(c))) + &(d)
    assert_eq!(unsauron!(a + b * c + d), a + b * c + d);
}

#[test]
fn block() {
    // Expands to:
    //
    // let x = &(5) + &(5);
    // let y = &(10) + &(10);
    // &(x) + &(&(y) * &(30))
    assert_eq!(unsauron!({
        let x = 5 + 5;
        let y = 10 + 10;
        x + y * 30
    }), 610);
}

// NOTE: !Copy
struct MyType;
impl<'a> Add<&'a MyType> for &'a MyType {
    type Output = MyType;

    fn add(self, _: &'a MyType) -> MyType {
        MyType
    }
}
#[test]
fn confirm_adding_refs() {
    let a = MyType;
    let b = MyType;
    unsauron!(a + b + a + a + b + b);
}
