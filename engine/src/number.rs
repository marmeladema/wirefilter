use lex::{expect, take_while, Lex, LexErrorKind, LexResult};

use std::str::FromStr;

fn number(input: &str, radix: u32) -> LexResult<u64> {
    let (digits, input) = take_while(input, "digit", |c| c.is_digit(radix))?;
    match u64::from_str_radix(digits, radix) {
        Ok(res) => Ok((res, input)),
        Err(err) => Err((LexErrorKind::ParseInt { err, radix }, digits)),
    }
}

impl<'i> Lex<'i> for u64 {
    fn lex(input: &str) -> LexResult<u64> {
        if let Ok(input) = expect(input, "0x") {
            number(input, 16)
        } else if let Ok(input) = expect(input, "0") {
            number(input, 8)
        } else {
            number(input, 10)
        }
    }
}

fn index(input: &str) -> LexResult<isize> {
    let (neg, input) = match expect(input, "-") {
        Ok(input) => (true, input),
        Err(_) => (false, input),
    };
    let (digits, input) = take_while(input, "digit", |c| c.is_digit(10))?;
    match isize::from_str(digits) {
        Ok(mut res) => {
            if neg {
                res = -res;
            }
            Ok((res, input))
        }
        Err(err) => Err((LexErrorKind::ParseInt { err, radix: 10 }, digits)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: isize,
    pub end: Option<isize>,
}

impl<'i> Lex<'i> for Range {
    fn lex(input: &str) -> LexResult<Self> {
        let (start, input) = if input.starts_with(':') {
            (0, input)
        } else {
            index(input)?
        };
        let (end, input) = if let Ok(input) = expect(input, ":") {
            match index(input) {
                Ok((len, input)) => (Some(start + len), input),
                Err(_) => (None, input),
            }
        } else if let Ok(input) = expect(input, "-") {
            let (end, input) = index(input)?;
            (Some(start + end - 1), input)
        } else {
            (None, input)
        };
        Ok((Range { start, end }, input))
    }
}

impl<'i> Lex<'i> for Vec<Range> {
    fn lex(input: &str) -> LexResult<Self> {
        let mut input = expect(input, "[")?;
        let mut res = Vec::new();
        loop {
            let (item, rest) = Range::lex(input)?;
            res.push(item);
            if let Ok(rest) = expect(rest.trim_left(), ",") {
                input = rest.trim_left();
            } else {
                return Ok((res, input));
            }
        }
    }
}