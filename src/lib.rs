#![no_std]
#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![allow(unused_variables)]

extern crate projectasiago_feta;

extern crate core as std;

#[macro_use]
extern crate nom;

named!(alt_tags, alt!(tag!("abcd") | tag!("efgh")));

///```
/// let result = projectasiago_mish::hello_world();
/// assert_eq!(result, "hi world fjdskl");
///```
pub fn hello_world() -> &'static str {
	assert_eq!(alt_tags(b"abcdxxx"), Ok((&b"xxx"[..], &b"abcd"[..])));
	assert_eq!(alt_tags(b"efghxxx"), Ok((&b"xxx"[..], &b"efgh"[..])));
	assert_eq!(alt_tags(b"ijklxxx"), Err(nom::Err::Error(error_position!(&b"ijklxxx"[..], nom::ErrorKind::Alt))));
	
	let s: &'static str = "hi world fjdskl";
	return s;
}