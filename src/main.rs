use std::env;
use std::str::Chars;

// skip white spce
// e.g. "   a" -> 'a'
fn extract_next_char(chars: &mut Chars) -> char {
    let c = chars.next().unwrap();
    if c.is_whitespace() {
        return extract_next_char(chars);
    }

    c
}

// "100" -> 100
fn conver_to_numeric(c: char, chars: &mut Chars) -> i32 {
    let mut val = c.to_digit(10).unwrap() as i32;

    for c in chars {
        if c.is_whitespace() {
            break;
        };

        if c.is_digit(10) {
            val = 10 * val + c.to_digit(10).unwrap() as i32;
        };
    }
    val
}

fn eval(chars: &mut Chars) -> i32 {
    let c = extract_next_char(chars);

    if c.is_digit(10) {
        let numeric = conver_to_numeric(c, chars);
        numeric
    } else {
        // Evaluation after '+|-|*|/' are evaluated
        let x = eval(chars);
        let y = eval(chars);

        match c {
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            '/' => x / y,
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let chars = &mut args[1].chars();

    eval(chars);
}

#[cfg(test)]
mod tests {
    use crate::eval;

    #[test]
    fn arithmetic_operation() {
        assert_eq!(eval(&mut "+ 30 20".chars()), 50);
        assert_eq!(eval(&mut "+ 40 + 10 20".chars()), 70);
        assert_eq!(eval(&mut "+ + 100 10 20".chars()), 130);
        assert_eq!(eval(&mut "+ 200 1".chars()), 201);
        assert_eq!(eval(&mut "- 30 20".chars()), 10);
        assert_eq!(eval(&mut "- 20 30".chars()), -10);
        assert_eq!(eval(&mut "* 10 10".chars()), 100);
        assert_eq!(eval(&mut "/ 100 5".chars()), 20);
    }
}
