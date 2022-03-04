use std::env;
use std::str::Chars;

// e.g. "0x20a" -> 'a'
fn extract_next_char(chars: &mut Chars) -> char {
    let c = chars.next().unwrap();
    if c.is_whitespace() {
        return extract_next_char(chars);
    }
    c
}

// e.g. ['1', '0', '0'] -> 100
fn conver_to_numeric(c: char, chars: &mut Chars) -> i32 {
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

fn eval_string(buffer: String, vec: &mut Vec<String>) -> i32 {
    let chars = &mut buffer.chars();
    let number = conver_to_numeric(chars.next().unwrap(), chars);

    let expression = vec.get(0).unwrap();

    eval(&mut expression.chars(), number, &mut vec.clone())
}

fn fn_apply(chars: &mut Chars, vec: &mut Vec<String>) -> i32 {
    let mut buffer = "".to_string();

    loop {
        let nextchar = chars.next().unwrap();
        match nextchar {
            '(' => (),
            ')' => break,
            'A'..='Z' => fn_apply(chars, vec)
                .to_string()
                .chars()
                .for_each(|c: char| buffer.push(c)),
            _ => buffer.push(nextchar),
        }
    }

    eval_string(buffer, vec)
}

fn fn_define(chars: &mut Chars, vec: &mut Vec<String>) {
    let mut buffer: String = "".to_string();

    loop {
        let c = chars.next().unwrap();
        match c {
            ']' => break,
            _ => buffer.push(c),
        }
    }
    vec.push(buffer);
}

fn eval(chars: &mut Chars, arg: i32, vec: &mut Vec<String>) -> i32 {
    let c = extract_next_char(chars);

    if c == '.' {
        chars.next();
        return arg;
    }

    if ('A'..='Z').contains(&c) {
        match chars.next().unwrap() {
            '[' => fn_define(chars, vec),
            '(' => return fn_apply(chars, vec),
            _ => unimplemented!(),
        }
    }
    if c.is_digit(10) {
        return conver_to_numeric(c, chars);
    }

    if c == '+' || c == '-' || c == '*' || c == '/' {
        let x = eval(chars, arg, vec);
        let y = eval(chars, arg, vec);
        return match c {
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            '/' => x / y,
            _ => {
                vec.pop();
                unimplemented!()
            }
        };
    };

    eval(chars, arg, vec)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec: Vec<String> = vec![];

    eval(&mut args[1].chars(), 0, &mut vec);
}

#[cfg(test)]
mod tests {
    use crate::eval;

    fn gen_vec() -> Vec<String> {
        vec![]
    }

    #[test]
    fn arithmetic_operation() {
        assert_eq!(eval(&mut "+ 30 20".chars(), 0, &mut gen_vec()), 50);
        assert_eq!(eval(&mut "+ 40 + 10 20".chars(), 0, &mut gen_vec()), 70);
        assert_eq!(eval(&mut "+ + 100 10 20".chars(), 0, &mut gen_vec()), 130);
        assert_eq!(eval(&mut "+ 200 1".chars(), 0, &mut gen_vec()), 201);
        assert_eq!(eval(&mut "- 30 20".chars(), 0, &mut gen_vec()), 10);
        assert_eq!(eval(&mut "- 20 30".chars(), 0, &mut gen_vec()), -10);
        assert_eq!(eval(&mut "* 10 10".chars(), 0, &mut gen_vec()), 100);
        assert_eq!(eval(&mut "/ 100 5".chars(), 0, &mut gen_vec()), 20);
    }
    #[test]
    fn function() {
        assert_eq!(eval(&mut "F[+ . .] F(1)".chars(), 0, &mut gen_vec()), 2);
        assert_eq!(eval(&mut "F[+ . .] F(10)".chars(), 0, &mut gen_vec()), 20);
        assert_eq!(eval(&mut "F[+ . 2] F(5)".chars(), 0, &mut gen_vec()), 7);
        assert_eq!(eval(&mut "F[* . 2] F(5)".chars(), 0, &mut gen_vec()), 10);
        assert_eq!(eval(&mut "F[* . .] F(10)".chars(), 0, &mut gen_vec()), 100);
        assert_eq!(eval(&mut "F[* . .] F(F(2))".chars(), 0, &mut gen_vec()), 16);
        assert_eq!(
            eval(&mut "F[* . .] F(F(F(2)))".chars(), 0, &mut gen_vec()),
            256
        );
        assert_eq!(
            eval(&mut "F[* . .] F(F(F(F(2))))".chars(), 0, &mut gen_vec()),
            65536
        );
    }
}
