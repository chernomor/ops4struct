Procedural macros to implements `Add`, `AddAssign`, `Sub`, `SubAssing` from `std::ops` for struct with named fields.


```
extern crate ops4struct;

use std::ops::Add;
use ops4struct::Add;

#[derive(Debug,PartialEq,Add,AddAssign,Sub,SubAssign)]
struct C {
	w: usize,
	r: u8,
}

fn main() {
	let a = C { w: 1, r: 0, };
	let b = C { w: 3, r: 4, };
	assert_eq!(C{w: 4, r: 4}, a+b);
}
```

Debug:
```
cargo install cargo-expand
cargo expand --example main
```
