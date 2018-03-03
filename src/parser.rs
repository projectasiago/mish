#![allow(unused_variables)]
#![allow(dead_code)]

use nom::Err::Error;
use nom::ErrorKind;
use std::vec::Vec;

named!(digit<char>, one_of!(&b"0123456789"[..]));
named!(decnumber<Vec<char> >, many1!(digit));

named!(lletter<char>, one_of!(&b"abcdefghijklmnopqrstuvwxyz"[..]));
named!(cletter<char>, one_of!(&b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..]));
named!(letter<char>, alt!(lletter | cletter));

#[derive(Debug, PartialEq)]
enum Expression {
	DecimalNumber(Vec<char>)
}

named!(expression<Expression>, alt!(
	decnumber => { |number: Vec<char>| Expression::DecimalNumber(number) } |
	subexpression => { |expression: Expression| expression }
));
named!(subexpression<Expression>, delimited!(char!('('), expression, char!(')')));

#[test]
pub fn test() {
	// digits
	assert_eq!(digit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(digit(b"a"), Err(Error(error_position!(&b"a"[..], ErrorKind::OneOf))));
	assert_eq!(decnumber(b"12 "), Ok((&b" "[..], vec!['1', '2'])));
	assert_eq!(decnumber(b"a "), Err(Error(error_position!(&b"a "[..], ErrorKind::Many1))));
	
	// letters
	assert_eq!(letter(b"a"), Ok((&b""[..], 'a')));
	assert_eq!(letter(b"0"), Err(Error(error_position!(&b"0"[..], ErrorKind::Alt))));
	
	// expression
	assert_eq!(expression(b"12 "), Ok((&b" "[..], Expression::DecimalNumber(vec!['1', '2']))));
	assert_eq!(expression(b"(12) "), Ok((&b" "[..], Expression::DecimalNumber(vec!['1', '2']))));
}