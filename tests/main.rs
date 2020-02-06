extern crate ops4struct;

use std::ops::{
	Add,
	AddAssign,
	Sub,
	SubAssign,
};
use ops4struct::{
	Add,
	AddAssign,
	Sub,
	SubAssign,
};

#[derive(Debug,PartialEq,Add,AddAssign,Sub,SubAssign)]
struct C {
	w: usize,
	r: u8,
}

#[test]
fn add() {
	let a = C { w: 1, r: 0, };
	let b = C { w: 3, r: 4, };
    assert_eq!(C{w: 4, r: 4}, a+b);
}
#[test]
fn add_assign() {
	let mut a = C { w: 1, r: 0, };
	let b = C { w: 3, r: 4, };
	a += b;
    assert_eq!(C{w: 4, r: 4}, a);
}
#[test]
fn sub() {
	let a = C { w: 4, r: 4, };
	let b = C { w: 3, r: 3, };
    assert_eq!(C{w: 1, r: 1}, a-b);
}
#[test]
fn sub_assign() {
	let mut a = C { w: 4, r: 4, };
	let b = C { w: 3, r: 3, };
	a -= b;
    assert_eq!(C{w: 1, r: 1}, a);
}
