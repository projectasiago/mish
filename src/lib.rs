#![no_std]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![allow(unused_variables)]

extern crate projectasiago_feta;

pub fn hello_world() -> &'static str {
	let s: &'static str = "hi world fjdskl";
	return s;
}