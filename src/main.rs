use core::panic;
use std::env;
use std::process::exit;
use std::str::Chars;

#[derive(Debug)]
enum ParseError {
    Eof,
    UnexpectedError,
}

// e.g. "0x20a" -> 'a'
fn extract_next_char(chars: &mut Chars) -> Result<char, ParseError> {
    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            return Ok(c);
        }
    }
    Err(ParseError::Eof)
}

// e.g. ['1', '0', '0'] -> 100
fn convert_to_numeric(c: char, chars: &mut Chars) -> i32 {
    let unhandling_to_i32 = |c: char| c.to_digit(10).unwrap() as i32;
    let mut val = unhandling_to_i32(c);

    for c in chars {
        if c.is_whitespace() {
            break;
        };

        if c.is_digit(10) {
            val = 10 * val + unhandling_to_i32(c)
        };
    }
    val
}

fn eval_string(buffer: String, vec: &mut Vec<String>) -> Result<i32, ParseError> {
    let chars = &mut buffer.chars();
    let number = convert_to_numeric(extract_next_char(chars)?, chars);

    let expression = vec.get(0).unwrap().clone();

    eval(&mut expression.chars(), number, vec)
}

fn fn_apply(chars: &mut Chars, vec: &mut Vec<String>) -> Result<i32, ParseError> {
    let mut buffer = "".to_string();

    loop {
        let nextchar = extract_next_char(chars)?;
        match nextchar {
            '(' => (),
            ')' => break,
            'f' if extract_next_char(chars)? == 'n' => {
                let ret = fn_apply(chars, vec)?;
                buffer.push_str(&ret.to_string());
            }
            _ => buffer.push(nextchar),
        }
    }

    eval_string(buffer, vec)
}

fn fn_define(chars: &mut Chars, vec: &mut Vec<String>) -> Result<(), ParseError> {
    let mut buffer = "".to_string();

    loop {
        let c = match chars.next() {
            Some(c) => Ok(c),
            None => Err(ParseError::Eof),
        }?;

        match c {
            ']' => break,
            _ => buffer.push(c),
        }
    }
    vec.push(buffer);
    Ok(())
}

fn eval(chars: &mut Chars, arg: i32, vec: &mut Vec<String>) -> Result<i32, ParseError> {
    let c = extract_next_char(chars)?;

    if c == '.' {
        chars.next();
        return Ok(arg);
    }

    if c == 'f' && extract_next_char(chars)? == 'n' {
        match extract_next_char(chars)? {
            '[' => fn_define(chars, vec)?,
            '(' => return fn_apply(chars, vec),
            _ => return Err(ParseError::UnexpectedError),
        }
    }
    if c.is_digit(10) {
        return Ok(convert_to_numeric(c, chars));
    }

    if c == '+' || c == '-' || c == '*' || c == '/' {
        let x = eval(chars, arg, vec)?;
        let y = eval(chars, arg, vec)?;
        return Ok(match c {
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            '/' => x / y,
            _ => {
                vec.pop();
                unimplemented!()
            }
        });
    };

    eval(chars, arg, vec)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec: Vec<String> = vec![];

    match eval(&mut args[1].chars(), 0, &mut vec) {
        Ok(_) => exit(0),
        Err(ParseError::Eof) => exit(0),
        Err(ParseError::UnexpectedError) => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::eval;

    fn gen_vec() -> Vec<String> {
        vec![]
    }

    #[test]
    fn arithmetic_operation() {
        assert_eq!(eval(&mut "+ 30 20".chars(), 0, &mut gen_vec()).unwrap(), 50);
        assert_eq!(
            eval(&mut "+ 40 + 10 20".chars(), 0, &mut gen_vec()).unwrap(),
            70
        );
        assert_eq!(
            eval(&mut "+ + 100 10 20".chars(), 0, &mut gen_vec()).unwrap(),
            130
        );
        assert_eq!(
            eval(&mut "+ 200 1".chars(), 0, &mut gen_vec()).unwrap(),
            201
        );
        assert_eq!(eval(&mut "- 30 20".chars(), 0, &mut gen_vec()).unwrap(), 10);
        assert_eq!(
            eval(&mut "- 20 30".chars(), 0, &mut gen_vec()).unwrap(),
            -10
        );
        assert_eq!(
            eval(&mut "* 10 10".chars(), 0, &mut gen_vec()).unwrap(),
            100
        );
        assert_eq!(eval(&mut "/ 100 5".chars(), 0, &mut gen_vec()).unwrap(), 20);
    }
    #[test]
    fn function() {
        assert_eq!(
            eval(&mut "fn[+ . .] fn(1)".chars(), 0, &mut gen_vec()).unwrap(),
            2
        );
        assert_eq!(
            eval(&mut "fn[+ . .] fn(10)".chars(), 0, &mut gen_vec()).unwrap(),
            20
        );
        assert_eq!(
            eval(&mut "fn[+ . 2] fn(5)".chars(), 0, &mut gen_vec()).unwrap(),
            7
        );
        assert_eq!(
            eval(&mut "fn[* . 2] fn(5)".chars(), 0, &mut gen_vec()).unwrap(),
            10
        );
        assert_eq!(
            eval(&mut "fn[* . .] fn(10)".chars(), 0, &mut gen_vec()).unwrap(),
            100
        );
        assert_eq!(
            eval(&mut "fn[* . .] fn(fn(2))".chars(), 0, &mut gen_vec()).unwrap(),
            16
        );
        assert_eq!(
            eval(&mut "fn[* . .] fn(fn(fn(2)))".chars(), 0, &mut gen_vec()).unwrap(),
            256
        );
        assert_eq!(
            eval(
                &mut "fn[* . .] fn(fn(fn(fn(2))))".chars(),
                0,
                &mut gen_vec()
            )
            .unwrap(),
            65536
        );
    }
}
