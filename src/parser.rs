#![allow(unused_variables)]
#![allow(dead_code)]

use std::vec::Vec;
use feta::RemoveWhere;
use feta::replace_lowercase_digits;

// TODO if prefix is empty, don't match it
#[macro_export]
macro_rules! base (
	($i:expr, $digits:expr, $type:ident, $prefix:expr) => {
		preceded!($i, tag!($prefix),
			map!(
				tuple!(
					map!(
						pair!(
							one_of!(&$digits[..]), // digit must start with a number
							map!(
								many0!(alt!(one_of!(&$digits[..]) | char!('_'))), // after that, can contain any number of digits or underscores
								|mut digits: Vec<char>| { digits.remove_where(|c: &char| c == &'_'); digits } // delete those underscores
							)
						),
						|mut value: (char, Vec<char>)| { // merge the first digit and the rest into one Vec
							value.1.insert(0, value.0);
							return value.1;
						}
					),
					opt!(preceded!( // optional decimal
						char!('.'),
						map!(
							many1!(alt!(one_of!(&$digits[..]) | char!('_'))), // can contain at least one digit or underscore
							|mut digits: Vec<char>| { digits.remove_where(|c: &char| c == &'_'); digits } // delete those underscores
						)
					))
				),
				|value: (Vec<char>, Option<Vec<char> >)| Expression::$type(value.0, match value.1 { // convert the optional decimal into an empty Vec
					Some(decimal_places) => decimal_places,
					None => vec![]
				})
			)
		);
	};
);

named!(decnumber<Expression>, base!("0123456789", DecNumber, ""));
named!(octnumber<Expression>, base!("01234567", OctNumber, "0o"));
named!(binnumber<Expression>, base!("01", BinNumber, "0b"));
named!(hexnumber<Expression>, map!(
	base!("0123456789ABCDEFabcdef", HexNumber, "0x"),
	|mut expression: Expression| {
		// replace all lowercase letters with uppercase ones
		match expression {
			Expression::HexNumber(ref mut first, ref mut second) => {
				replace_lowercase_digits(first);
				replace_lowercase_digits(second);
			},
			_ => unreachable!(),
		}
		
		return expression;
	}
));

named!(lletter<char>, one_of!(&b"abcdefghijklmnopqrstuvwxyz"[..]));
named!(cletter<char>, one_of!(&b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..]));
named!(letter<char>, alt!(lletter | cletter));

#[derive(Debug, PartialEq)]
enum BinaryOperation {
	Add(Expression),
	Subtract(Expression),
	Multiply(Expression),
	Divide(Expression),
}

#[derive(Debug, PartialEq)]
enum Expression {
	DecNumber(Vec<char>, Vec<char>),
	HexNumber(Vec<char>, Vec<char>),
	OctNumber(Vec<char>, Vec<char>),
	BinNumber(Vec<char>, Vec<char>),
	BinaryOperation(Box<Expression>, Vec<BinaryOperation>),
}

named!(add<BinaryOperation>, ws!(do_parse!(
	char!('+') >>
	expression: expr_level1 >>
	(BinaryOperation::Add(expression))
)));
named!(subtract<BinaryOperation>, ws!(do_parse!(
	char!('-') >>
	expression: expr_level1 >>
	(BinaryOperation::Subtract(expression))
)));

named!(multiply<BinaryOperation>, ws!(do_parse!(
	char!('*') >>
	expression: expr_level2 >>
	(BinaryOperation::Multiply(expression))
)));
named!(divide<BinaryOperation>, ws!(do_parse!(
	char!('/') >>
	expression: expr_level2 >>
	(BinaryOperation::Divide(expression))
)));

fn binop(first: Expression, operations: Vec<BinaryOperation>) -> Expression {
	if operations.len() == 0 {
		first
	} else {
		Expression::BinaryOperation(Box::new(first), operations)
	}
}

named!(expression<Expression>, ws!(do_parse!(
	first: expr_level1 >>
	operations: many0!(alt!(
		add |
		subtract
	)) >>
	(binop(first, operations))
)));

named!(expr_level1<Expression>, ws!(do_parse!(
	first: expr_level2 >>
	operations: many0!(alt!(
		multiply |
		divide
	)) >>
	(binop(first, operations))
)));

named!(expr_level2<Expression>, ws!(alt!(
	hexnumber | // hex, oct, and bin must come first to prevent the number from being interpreted as decimal
	octnumber |
	binnumber |
	decnumber |
	subexpression
)));
named!(subexpression<Expression>, delimited!(char!('('), expression, char!(')')));

#[test]
pub fn test_numbers() {
	use nom::Err::Error;
	use nom::ErrorKind;
	
	// single digits
	assert_eq!(decnumber(b"1x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1'], vec![]))));
	assert_eq!(hexnumber(b"0x1x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1'], vec![]))));
	assert_eq!(octnumber(b"0o1x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1'], vec![]))));
	assert_eq!(binnumber(b"0b1x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1'], vec![]))));
	
	// double digits
	assert_eq!(decnumber(b"11x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '1'], vec![]))));
	assert_eq!(hexnumber(b"0x11x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1', '1'], vec![]))));
	assert_eq!(octnumber(b"0o11x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1', '1'], vec![]))));
	assert_eq!(binnumber(b"0b11x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1', '1'], vec![]))));
	
	// decimal places
	assert_eq!(decnumber(b"11.1x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(hexnumber(b"0x11.1x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(octnumber(b"0o11.1x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(binnumber(b"0b11.1x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1', '1'], vec!['1']))));
	
	// underscores
	assert_eq!(decnumber(b"1_1.1x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(hexnumber(b"0x1_1.1x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(octnumber(b"0o1_1.1x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(binnumber(b"0b1_1.1x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1', '1'], vec!['1']))));
	assert_eq!(decnumber(b"1_.1x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1'], vec!['1']))));
	assert_eq!(hexnumber(b"0x1_.1x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1'], vec!['1']))));
	assert_eq!(octnumber(b"0o1_.1x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1'], vec!['1']))));
	assert_eq!(binnumber(b"0b1_.1x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1'], vec!['1']))));
	assert_eq!(decnumber(b"1._1x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1'], vec!['1']))));
	assert_eq!(hexnumber(b"0x1._1x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1'], vec!['1']))));
	assert_eq!(octnumber(b"0o1._1x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1'], vec!['1']))));
	assert_eq!(binnumber(b"0b1._1x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1'], vec!['1']))));
	assert_eq!(decnumber(b"1.1_x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1'], vec!['1']))));
	assert_eq!(hexnumber(b"0x1.1_x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1'], vec!['1']))));
	assert_eq!(octnumber(b"0o1.1_x"), Ok((&b"x"[..], Expression::OctNumber(vec!['1'], vec!['1']))));
	assert_eq!(binnumber(b"0b1.1_x"), Ok((&b"x"[..], Expression::BinNumber(vec!['1'], vec!['1']))));
	
	// other tests
	assert_eq!(decnumber(b"12.x"), Ok((&b".x"[..], Expression::DecNumber(vec!['1', '2'], vec![]))));
	assert_eq!(decnumber(b"1 2x"), Ok((&b" 2x"[..], Expression::DecNumber(vec!['1'], vec![]))));
	assert_eq!(decnumber(b"ax"), Err(Error(error_position!(&b"ax"[..], ErrorKind::OneOf))));
	
	// hex gets capitalized
	assert_eq!(hexnumber(b"0xax"), Ok((&b"x"[..], Expression::HexNumber(vec!['A'], vec![]))));
	assert_eq!(hexnumber(b"0xAx"), Ok((&b"x"[..], Expression::HexNumber(vec!['A'], vec![]))));
	assert_eq!(hexnumber(b"0x5x"), Ok((&b"x"[..], Expression::HexNumber(vec!['5'], vec![]))));
}

#[test]
pub fn test_letters() {
	use nom::Err::Error;
	use nom::ErrorKind;
	
	assert_eq!(letter(b"a"), Ok((&b""[..], 'a')));
	assert_eq!(letter(b"0"), Err(Error(error_position!(&b"0"[..], ErrorKind::Alt))));
}

#[test]
pub fn test_expression() {
	use nom::Err::Incomplete;
	use nom::Needed::Size;
	
	assert_eq!(expression(b"12x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b"(12)x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b" 12 x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b" ( 12 ) x"), Ok((&b"x"[..], Expression::DecNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b"0x12x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b"(0x12)x"), Ok((&b"x"[..], Expression::HexNumber(vec!['1', '2'], vec![]))));
	assert_eq!(expression(b"(5"), Err(Incomplete(Size(1))));
	
	// adding
	assert_eq!(expression(b"2+4x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![BinaryOperation::Add(Expression::DecNumber(vec!['4'], vec![]))],
	))));
	assert_eq!(expression(b" 2 + 4 x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![BinaryOperation::Add(Expression::DecNumber(vec!['4'], vec![]))],
	))));
	
	// multiplying
	assert_eq!(expression(b"2*4x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![BinaryOperation::Multiply(Expression::DecNumber(vec!['4'], vec![]))],
	))));
	assert_eq!(expression(b" 2 * 4 x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![BinaryOperation::Multiply(Expression::DecNumber(vec!['4'], vec![]))],
	))));
	
	// multi
	assert_eq!(expression(b"2+4+8x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![
			BinaryOperation::Add(Expression::DecNumber(vec!['4'], vec![])),
			BinaryOperation::Add(Expression::DecNumber(vec!['8'], vec![])),
		],
	))));
	assert_eq!(expression(b"2*4*8x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![
			BinaryOperation::Multiply(Expression::DecNumber(vec!['4'], vec![])),
			BinaryOperation::Multiply(Expression::DecNumber(vec!['8'], vec![])),
		],
	))));
	
	// add and multiply
	assert_eq!(expression(b"2+4*8x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![BinaryOperation::Add(Expression::BinaryOperation(
			Box::new(Expression::DecNumber(vec!['4'], vec![])),
			vec![
				BinaryOperation::Multiply(Expression::DecNumber(vec!['8'], vec![])),
			],
		))],
	))));
	assert_eq!(expression(b"2*4+8x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::BinaryOperation(
			Box::new(Expression::DecNumber(vec!['2'], vec![])),
			vec![
				BinaryOperation::Multiply(Expression::DecNumber(vec!['4'], vec![])),
			],
		)),
		vec![BinaryOperation::Add(Expression::DecNumber(vec!['8'], vec![]))],
	))));
	assert_eq!(expression(b"2*4+8*16x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::BinaryOperation(
			Box::new(Expression::DecNumber(vec!['2'], vec![])),
			vec![
				BinaryOperation::Multiply(Expression::DecNumber(vec!['4'], vec![])),
			],
		)),
		vec![
			BinaryOperation::Add(Expression::BinaryOperation(
				Box::new(Expression::DecNumber(vec!['8'], vec![])),
				vec![
					BinaryOperation::Multiply(Expression::DecNumber(vec!['1', '6'], vec![])),
				],
			))
		],
	))));
	assert_eq!(expression(b"2+4*8+16x"), Ok((&b"x"[..], Expression::BinaryOperation(
		Box::new(Expression::DecNumber(vec!['2'], vec![])),
		vec![
			BinaryOperation::Add(Expression::BinaryOperation(
				Box::new(Expression::DecNumber(vec!['4'], vec![])),
				vec![
					BinaryOperation::Multiply(Expression::DecNumber(vec!['8'], vec![])),
				],
			)),
			BinaryOperation::Add(Expression::DecNumber(vec!['1', '6'], vec![]))
		],
	))));
}