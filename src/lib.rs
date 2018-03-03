#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![allow(unused_variables)]

#![cfg_attr(feature = "no_std", no_std)]
#![cfg_attr(feature = "no_std", feature(alloc))]
#[cfg(feature = "no_std")]
extern crate alloc;

#[cfg(feature = "no_std")]
mod std {
	pub use alloc::{boxed, string, vec};
	
	pub use core::{cmp, convert, fmt, iter, mem, ops, option, result, slice, str};
	
	pub mod prelude {
		pub use core::prelude as v1;
	}
}

extern crate projectasiago_feta as feta;

#[macro_use]
extern crate nom;

mod parser;

///```
/// let result = projectasiago_mish::hello_world();
/// assert_eq!(result, "hi world fjdskl");
///```
pub fn hello_world() -> &'static str {
//assert_eq!(alt_tags(b"abcdxxx"), Ok((&b"xxx"[..], &b"abcd"[..])));
//assert_eq!(alt_tags(b"efghxxx"), Ok((&b"xxx"[..], &b"efgh"[..])));
//assert_eq!(alt_tags(b"ijklxxx"), Err(nom::Err::Error(error_position!(&b"ijklxxx"[..], nom::ErrorKind::Alt))));
	
	//parser::do_parse();
	
	let s: &'static str = "hi world fjdskl";
	return s;
}