#![allow(unused_variables)]
#![allow(dead_code)]

use nom;

named!(digit<char>, one_of!(&b"0123456789"[..]));
named!(decnumber<Vec<char> >, many1!(digit));

named!(lletter<char>, one_of!(&b"abcdefghijklmnopqrstuvwxyz"[..]));
named!(cletter<char>, one_of!(&b"ABCDEFGHIJKLMNOPQRSTUVWXYZ"[..]));
named!(letter<char>, alt!(lletter | cletter));

#[test]
pub fn test() {
	// digits
	assert_eq!(digit(b"1"), Ok((&b""[..], '1')));
	assert_eq!(digit(b"a"), Err(nom::Err::Error(error_position!(&b"a"[..], nom::ErrorKind::OneOf))));
	assert_eq!(decnumber(b"12 "), Ok((&b" "[..], vec!['1', '2'])));
	assert_eq!(decnumber(b"a "), Err(nom::Err::Error(error_position!(&b"a "[..], nom::ErrorKind::Many1))));
	
	// letters
	assert_eq!(letter(b"a"), Ok((&b""[..], 'a')));
	assert_eq!(letter(b"0"), Err(nom::Err::Error(error_position!(&b"0"[..], nom::ErrorKind::Alt))));
}