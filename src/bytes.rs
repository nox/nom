//! Byte level parsers and combinators
//!

/// `tag!(&[T]: nom::AsBytes) => &[T] -> IResult<&[T], &[T]>`
/// declares a byte array as a suite to recognize
///
/// consumes the recognized characters
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
/// # fn main() {
///  named!(x, tag!("abcd"));
///  let r = x(&b"abcdefgh"[..]);
///  assert_eq!(r, Done(&b"efgh"[..], &b"abcd"[..]));
/// # }
/// ```
#[macro_export]
macro_rules! tag (
  ($i:expr, $inp: expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected = $inp;
      let bytes = as_bytes(&expected);

      if bytes.len() > $i.len() {
        $crate::IResult::Incomplete($crate::Needed::Size(bytes.len()))
      } else if &$i[0..bytes.len()] == bytes {
        $crate::IResult::Done(&$i[bytes.len()..], &$i[0..bytes.len()])
      } else {
        $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::Tag, $i))
      }
    }
  );
);

/// `is_not!(&[T:AsBytes]) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes that do not appear in the provided array
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
/// # fn main() {
///  named!( not_space, is_not!( " \t\r\n" ) );
///
///  let r = not_space(&b"abcdefgh\nijkl"[..]);
///  assert_eq!(r, Done(&b"\nijkl"[..], &b"abcdefgh"[..]));
///  # }
/// ```
#[macro_export]
macro_rules! is_not(
  ($input:expr, $arr:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected   = $arr;
      let bytes      = as_bytes(&expected);
      let mut parsed = false;
      let mut index  = 0;

      for idx in 0..$input.len() {
        index = idx;
        for &i in bytes.iter() {
          if $input[idx] == i {
            parsed = true;
            break;
          }
        }
        if parsed { break; }
      }
      if index == 0 {
        $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::IsNot,$input))
      } else {
        $crate::IResult::Done(&$input[index..], &$input[0..index])
      }
    }
  );
);

/// `is_a!(&[T]) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes that appear in the provided array
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
/// # fn main() {
///  named!(abcd, is_a!( "abcd" ));
///
///  let r1 = abcd(&b"aaaaefgh"[..]);
///  assert_eq!(r1, Done(&b"efgh"[..], &b"aaaa"[..]));
///
///  let r2 = abcd(&b"dcbaefgh"[..]);
///  assert_eq!(r2, Done(&b"efgh"[..], &b"dcba"[..]));
/// # }
/// ```
#[macro_export]
macro_rules! is_a(
  ($input:expr, $arr:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected  = $arr;
      let bytes     = as_bytes(&expected);
      let mut index = 0;

      for idx in 0..$input.len() {
        index = idx;
        let mut cont = false;
        for &i in bytes.iter() {
          if $input[idx] == i {
            cont = true;
            break;
          }
        }
        if !cont { break; }
      }
      if index == 0 {
        $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::IsA,$input))
      } else {
        $crate::IResult::Done(&$input[index..], &$input[0..index])
      }
    }
  );
);

/// `filter!(&[T] -> bool) => &[T] -> IResult<&[T], &[T]>`
/// returns the longest list of bytes until the provided function fails.
///
/// The argument is either a function `&[T] -> bool` or a macro returning a `bool
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
/// # use nom::is_alphanumeric;
/// # fn main() {
///  named!( alpha, filter!( is_alphanumeric ) );
///
///  let r = alpha(&b"abcd\nefgh"[..]);
///  assert_eq!(r, Done(&b"\nefgh"[..], &b"abcd"[..]));
/// # }
/// ```
#[macro_export]
macro_rules! filter(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (

    {
      let mut index = 0;
      let mut found = false;
      for idx in 0..$input.len() {
        index = idx;
        if !$submac!($input[idx], $($args)*) {
          found = true;
          break;
        }
      }
      if index == 0 {
        $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::Filter,$input))
      } else if found {
        $crate::IResult::Done(&$input[index..], &$input[0..index])
      } else {
        $crate::IResult::Done(&b""[..], $input)
      }
    }
  );
  ($input:expr, $f:expr) => (
    filter!($input, call!($f));
  );
);

/// `take!(nb) => &[T] -> IResult<&[T], &[T]>`
/// generates a parser consuming the specified number of bytes
///
/// ```
/// # #[macro_use] extern crate nom;
/// # use nom::IResult::Done;
/// # fn main() {
///  // Desmond parser
///  named!(take5, take!( 5 ) );
///
///  let a = b"abcdefgh";
///
///  assert_eq!(take5(&a[..]), Done(&b"fgh"[..], &b"abcde"[..]));
/// # }
/// ```
#[macro_export]
macro_rules! take(
  ($i:expr, $count:expr) => (
    {
      let cnt = $count as usize;
      if $i.len() < cnt {
        $crate::IResult::Incomplete($crate::Needed::Size(cnt))
      } else {
        $crate::IResult::Done(&$i[cnt..],&$i[0..cnt])
      }
    }
  );
);

/// `take!(nb) => &[T] -> IResult<&[T], &str>`
/// same as take! but returning a &str
#[macro_export]
macro_rules! take_str (
 ( $i:expr, $size:expr ) => ( map_res!($i, take!($size), from_utf8) );
);

/// `take_until_and_consume!(tag) => &[T] -> IResult<&[T], &[T]>`
/// generates a parser consuming bytes until the specified byte sequence is found, and consumes it
#[macro_export]
macro_rules! take_until_and_consume(
  ($i:expr, $inp:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected   = $inp;
      let bytes      = as_bytes(&expected);
      if bytes.len() > $i.len() {
        $crate::IResult::Incomplete($crate::Needed::Size(bytes.len()))
      } else {
        let mut index  = 0;
        let mut parsed = false;

        for idx in 0..$i.len() {
          if idx + bytes.len() > $i.len() {
            index = idx;
            break;
          }
          if &$i[idx..idx + bytes.len()] == bytes {
            parsed = true;
            index = idx;
            break;
          }
        }

        if parsed {
          $crate::IResult::Done(&$i[(index + bytes.len())..], &$i[0..index])
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::TakeUntilAndConsume,$i))
        }
      }
    }
  );
);

/// `take_until!(tag) => &[T] -> IResult<&[T], &[T]>`
/// consumes data until it finds the specified tag
#[macro_export]
macro_rules! take_until(
  ($i:expr, $inp:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected   = $inp;
      let bytes      = as_bytes(&expected);
      if bytes.len() > $i.len() {
        $crate::IResult::Incomplete($crate::Needed::Size(bytes.len()))
      } else {
        let mut index  = 0;
        let mut parsed = false;

        for idx in 0..$i.len() {
          if idx + bytes.len() > $i.len() {
            index = idx;
            break;
          }
          if &$i[idx..idx+bytes.len()] == bytes {
            parsed = true;
            index  = idx;
            break;
          }
        }

        if parsed {
          $crate::IResult::Done(&$i[index..], &$i[0..index])
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::TakeUntil,$i))
        }
      }
    }
  );
);

/// `take_until_either_and_consume!(tag) => &[T] -> IResult<&[T], &[T]>`
/// consumes data until it finds any of the specified characters, and consume it
#[macro_export]
macro_rules! take_until_either_and_consume(
  ($i:expr, $inp:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected   = $inp;
      let bytes      = as_bytes(&expected);
      if 1 > $i.len() {
        $crate::IResult::Incomplete($crate::Needed::Size(1))
      } else {
        let mut index  = 0;
        let mut parsed = false;

        for idx in 0..$i.len() {
          if idx + 1 > $i.len() {
            index = idx;
            break;
          }
          for &t in bytes.iter() {
            if $i[idx] == t {
              parsed = true;
              index = idx;
              break;
            }
          }
          if parsed { break; }
        }

        if parsed {
          $crate::IResult::Done(&$i[(index+1)..], &$i[0..index])
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::TakeUntilEitherAndConsume,$i))
        }
      }
    }
  );
);

/// `take_until_either!(tag) => &[T] -> IResult<&[T], &[T]>`
#[macro_export]
macro_rules! take_until_either(
  ($i:expr, $inp:expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected   = $inp;
      let bytes      = as_bytes(&expected);
      if 1 > $i.len() {
        $crate::IResult::Incomplete($crate::Needed::Size(1))
      } else {
        let mut index  = 0;
        let mut parsed = false;

        for idx in 0..$i.len() {
          if idx + 1 > $i.len() {
            index = idx;
            break;
          }
          for &t in bytes.iter() {
            if $i[idx] == t {
              parsed = true;
              index = idx;
              break;
            }
          }
          if parsed { break; }
        }

        if parsed {
          $crate::IResult::Done(&$i[index..], &$i[0..index])
        } else {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::TakeUntilEither,$i))
        }
      }
    }
  );
);

#[cfg(test)]
mod tests {
  use internal::Needed;
  use internal::IResult::*;
  use internal::Err::*;
  use util::ErrorKind;

  #[test]
  fn is_a() {
    named!(a_or_b, is_a!(&b"ab"[..]));

    let a = &b"abcd"[..];
    assert_eq!(a_or_b(a), Done(&b"cd"[..], &b"ab"[..]));

    let b = &b"bcde"[..];
    assert_eq!(a_or_b(b), Done(&b"cde"[..], &b"b"[..]));

    let c = &b"cdef"[..];
    assert_eq!(a_or_b(c), Error(Position(ErrorKind::IsA,c)));

    let d = &b"bacdef"[..];
    assert_eq!(a_or_b(d), Done(&b"cdef"[..], &b"ba"[..]));
  }

  use std::str::from_utf8;
  #[test]
  fn take_str_test() {
    let a = b"omnomnom";

    assert_eq!(take_str!(&a[..], 5), Done(&b"nom"[..], "omnom"));
    assert_eq!(take_str!(&a[..], 9), Incomplete(Needed::Size(9)));
  }

  #[test]
  fn take_until_test() {
    named!(x, take_until_and_consume!("efgh"));
    let r = x(&b"abcdabcdefghijkl"[..]);
    assert_eq!(r, Done(&b"ijkl"[..], &b"abcdabcd"[..]));

    println!("Done 1\n");

    let r2 = x(&b"abcdabcdefgh"[..]);
    assert_eq!(r2, Done(&b""[..], &b"abcdabcd"[..]));

    println!("Done 2\n");
    let r3 = x(&b"abcefg"[..]);
    assert_eq!(r3,  Error(Position(ErrorKind::TakeUntilAndConsume, &b"abcefg"[..])));

    assert_eq!(
      x(&b"ab"[..]),
      Incomplete(Needed::Size(4))
    );
  }

  #[test]
  fn take_until_either_incomplete() {
    named!(x, take_until_either!("!."));
    assert_eq!(
      x(&b"123"[..]),
      Error(Position(ErrorKind::TakeUntilEither, &b"123"[..]))
    );
  }

  #[test]
  fn take_until_incomplete() {
    named!(y, take_until!("end"));
    assert_eq!(
      y(&b"nd"[..]),
      Incomplete(Needed::Size(3))
    );
    assert_eq!(
      y(&b"123"[..]),
      Error(Position(ErrorKind::TakeUntil, &b"123"[..]))
    );
  }
}
