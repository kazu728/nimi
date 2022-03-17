use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Function,
    LBracket,
    RBracket,
    LParen,
    Rparen,
    Dot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LexError {
    Unexpected,
    Eof,
}

// e.g. "0x20a" -> 'a'
fn extract_next_char(chars: &mut Peekable<Chars>) -> Result<char, LexError> {
    for c in chars {
        if !c.is_whitespace() {
            return Ok(c);
        }
    }
    Err(LexError::Eof)
}

// e.g. ['1', '0', '0'] -> 100
fn convert_to_numeric(c: &char, chars: &mut Peekable<Chars>) -> Result<i32, LexError> {
    let char_to_i32 = |c: char| c.to_digit(10).ok_or(LexError::Unexpected).map(|n| n as i32);

    let mut val = char_to_i32(*c)?;

    while let Some(c) = chars.clone().peek() {
        if c.is_digit(10) {
            val = 10 * val + char_to_i32(*c)?;
            chars.next();
        } else {
            break;
        }
    }

    Ok(val)
}

fn lex(input: &str) -> Result<Vec<TokenKind>, LexError> {
    let mut tokens: Vec<TokenKind> = vec![];

    let mut chars = input.chars().into_iter().peekable();

    loop {
        match chars.peek() {
            Some(char) => {
                let token = match extract_next_char(&mut chars) {
                    Ok('+') => TokenKind::Plus,
                    Ok('-') => TokenKind::Minus,
                    Ok('*') => TokenKind::Asterisk,
                    Ok('/') => TokenKind::Slash,
                    Ok('[') => TokenKind::LBracket,
                    Ok(']') => TokenKind::RBracket,
                    Ok('(') => TokenKind::LParen,
                    Ok(')') => TokenKind::Rparen,
                    Ok('.') => TokenKind::Dot,
                    Ok('f') => {
                        chars.next();
                        TokenKind::Function
                    }
                    Ok(n) => TokenKind::Number(convert_to_numeric(&n, &mut chars)?),
                    _ => unreachable!(""),
                };
                tokens.push(token)
            }
            None => break,
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::lexer::{lex, TokenKind};

    #[test]
    fn test_lexer_arithmetic() {
        assert_eq!(
            lex("+ 30 20"),
            Ok(vec![
                TokenKind::Plus,
                TokenKind::Number(30),
                TokenKind::Number(20),
            ])
        );

        assert_eq!(
            lex("+ 40 + 10 20"),
            Ok(vec![
                TokenKind::Plus,
                TokenKind::Number(40),
                TokenKind::Plus,
                TokenKind::Number(10),
                TokenKind::Number(20)
            ])
        );

        assert_eq!(
            lex("- 20 30"),
            Ok(vec![
                TokenKind::Minus,
                TokenKind::Number(20),
                TokenKind::Number(30)
            ])
        )
    }

    #[test]
    fn test_lexer_function() {
        assert_eq!(
            lex("fn[+ . .] fn(1)"),
            Ok(vec![
                TokenKind::Function,
                TokenKind::LBracket,
                TokenKind::Plus,
                TokenKind::Dot,
                TokenKind::Dot,
                TokenKind::RBracket,
                TokenKind::Function,
                TokenKind::LParen,
                TokenKind::Number(1),
                TokenKind::Rparen,
            ])
        );

        assert_eq!(
            lex("fn[* . .] fn(fn(fn(fn(2))))"),
            Ok(vec![
                TokenKind::Function,
                TokenKind::LBracket,
                TokenKind::Asterisk,
                TokenKind::Dot,
                TokenKind::Dot,
                TokenKind::RBracket,
                TokenKind::Function,
                TokenKind::LParen,
                TokenKind::Function,
                TokenKind::LParen,
                TokenKind::Function,
                TokenKind::LParen,
                TokenKind::Function,
                TokenKind::LParen,
                TokenKind::Number(2),
                TokenKind::Rparen,
                TokenKind::Rparen,
                TokenKind::Rparen,
                TokenKind::Rparen,
            ])
        );
    }
}
