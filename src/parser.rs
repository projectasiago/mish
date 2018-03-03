#![allow(unused_variables)]
#![allow(dead_code)]

use std::vec::Vec;

named!(decdigit<char>, one_of!(&b"0123456789"[..]));
named!(decnumber<Vec<char> >, many1!(decdigit));

named!(hexdigit<char>, one_of!(&b"0123456789ABCDEFabcdef"[..]));
named!(hexnumber<Vec<char> >, preceded!(tag!("0x"), many1!(hexdigit)));

named!(octdigit<char>, one_of!(&b"01234567"[..]));
named!(octnumber<Vec<char> >, preceded!(tag!("0o"), many1!(octdigit)));

named!(bindigit<char>, one_of!(&b"01"[..]));
named!(binnumber<Vec<char> >, preceded!(tag!("0b"), many1!(bindigit)));

named!(lletter<char>, one_of!(&b"abcdefghijklmnopqrstuvwxyz"[..]));
named!(cletter<char>, one_of!(&b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..]));
named!(letter<char>, alt!(lletter | cletter));

#[derive(Debug, PartialEq)]
enum Expression {
	DecimalNumber(Vec<char>),
	HexNumber(Vec<char>),
	OctNumber(Vec<char>),
	BinNumber(Vec<char>),
}

named!(expression<Expression>, alt!(
	hexnumber => { |number: Vec<char>| Expression::HexNumber(number) } | // hex and others must come first to prevent the number from being interpreted as decimal
	octnumber => { |number: Vec<char>| Expression::OctNumber(number) } |
	binnumber => { |number: Vec<char>| Expression::BinNumber(number) } |
	decnumber => { |number: Vec<char>| Expression::DecimalNumber(number) } |
	subexpression => { |expression: Expression| expression }
));
named!(subexpression<Expression>, ws!(delimited!(char!('('), expression, char!(')'))));

#[test]
pub fn test() {
	use nom::Err::Error;
	use nom::ErrorKind;
	
	// decimal numbers
	assert_eq!(decdigit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(decdigit(b"a"), Err(Error(error_position!(&b"a"[..], ErrorKind::OneOf))));
	assert_eq!(decnumber(b"12x"), Ok((&b"x"[..], vec!['1', '2'])));
	assert_eq!(decnumber(b"1 2x"), Ok((&b" 2x"[..], vec!['1'])));
	assert_eq!(decnumber(b"ax"), Err(Error(error_position!(&b"ax"[..], ErrorKind::Many1))));
	
	// hexadecimal numbers
	assert_eq!(hexdigit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(hexdigit(b"l"), Err(Error(error_position!(&b"l"[..], ErrorKind::OneOf))));
	assert_eq!(hexnumber(b"0x12x"), Ok((&b"x"[..], vec!['1', '2'])));
	assert_eq!(hexnumber(b"0x1 2x"), Ok((&b" 2x"[..], vec!['1'])));
	assert_eq!(hexnumber(b"ax"), Err(Error(error_position!(&b"ax"[..], ErrorKind::Tag))));
	assert_eq!(hexnumber(b"0x "), Err(Error(error_position!(&b" "[..], ErrorKind::Many1))));
	
	// octal numbers
	assert_eq!(octdigit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(octdigit(b"l"), Err(Error(error_position!(&b"l"[..], ErrorKind::OneOf))));
	assert_eq!(octnumber(b"0o12x"), Ok((&b"x"[..], vec!['1', '2'])));
	assert_eq!(octnumber(b"0o1 2x"), Ok((&b" 2x"[..], vec!['1'])));
	assert_eq!(octnumber(b"ax"), Err(Error(error_position!(&b"ax"[..], ErrorKind::Tag))));
	assert_eq!(octnumber(b"0o "), Err(Error(error_position!(&b" "[..], ErrorKind::Many1))));
	
	// binary numbers
	assert_eq!(bindigit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(bindigit(b"l"), Err(Error(error_position!(&b"l"[..], ErrorKind::OneOf))));
	assert_eq!(binnumber(b"0b01x"), Ok((&b"x"[..], vec!['0', '1'])));
	assert_eq!(binnumber(b"0b1 2x"), Ok((&b" 2x"[..], vec!['1'])));
	assert_eq!(binnumber(b"ax"), Err(Error(error_position!(&b"ax"[..], ErrorKind::Tag))));
	assert_eq!(binnumber(b"0b "), Err(Error(error_position!(&b" "[..], ErrorKind::Many1))));
	
	// letters
	assert_eq!(letter(b"a"), Ok((&b""[..], 'a')));
	assert_eq!(letter(b"0"), Err(Error(error_position!(&b"0"[..], ErrorKind::Alt))));
	
	// expression
	assert_eq!(expression(b"12x"), Ok((&b"x"[..], Expression::DecimalNumber(vec!['1', '2']))));
	assert_eq!(expression(b"0xAFx"), Ok((&b"x"[..], Expression::HexNumber(vec!['A', 'F']))));
	assert_eq!(expression(b"0o67x"), Ok((&b"x"[..], Expression::OctNumber(vec!['6', '7']))));
	assert_eq!(expression(b"0b01x"), Ok((&b"x"[..], Expression::BinNumber(vec!['0', '1']))));
	assert_eq!(expression(b"(12)x"), Ok((&b"x"[..], Expression::DecimalNumber(vec!['1', '2']))));
	assert_eq!(expression(b" ( 12 ) x"), Ok((&b"x"[..], Expression::DecimalNumber(vec!['1', '2']))));
}