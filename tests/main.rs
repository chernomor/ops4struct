extern crate add_struct;

use std::ops::Add;
use std::ops::AddAssign;
use add_struct::Add;
use add_struct::AddAssign;

#[derive(Debug,PartialEq,Add,AddAssign)]
struct C {
	w: usize,
	e: f32,
	r: u8,
}

#[test]
fn add() {
	let a = C { w: 1, e: 2.2, r: 0, };
	let b = C { w: 3, e: 3.2, r: 4, };
    assert_eq!(C{w: 4, e: 5.4, r: 4}, a+b);
}
#[test]
fn add_assign() {
	let mut a = C { w: 1, e: 2.2, r: 0, };
	let b = C { w: 3, e: 3.2, r: 4, };
	a += b;
    assert_eq!(C{w: 4, e: 5.4, r: 4}, a);
}
