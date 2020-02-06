// vim: set expandtab ts=4 sw=4:
extern crate add_struct;

use std::ops::{
	Add,
};
use add_struct::{
	Add,
};

#[derive(Debug,PartialEq,Add)]
struct C {
	w: usize,
	e: f32,
	r: u8,
}

fn main() {
	let a = C { w: 1, e: 2.2, r: 0, };
	let b = C { w: 3, e: 3.2, r: 4, };
    dbg!(C{w: 4, e: 5.4, r: 4}, a+b);
}

