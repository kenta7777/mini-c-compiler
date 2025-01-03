pub mod tokenizer {
    use std::{char, iter::Peekable, str::Chars};

    #[derive(Debug)]
    pub enum TokenKind {
        Plus,
        Minus,
        Number(u32),
    }

    pub enum TokenizeError {
        UnexpectedToken,
    }

    pub fn tokenize(input: &String) -> Result<Vec<TokenKind>, TokenizeError> {
        let mut tokens: Vec<TokenKind> = Vec::new();
        let mut chars = (*input).chars().peekable();
        while let Some(&c) = chars.peek() {
            match c {
                ' ' => {
                    chars.next();
                }
                '+' => {
                    chars.next();
                    tokens.push(TokenKind::Plus);
                }
                '-' => {
                    chars.next();
                    tokens.push(TokenKind::Minus);
                }
                '0'..='9' => match tokenize_number(&mut chars) {
                    Ok(token) => tokens.push(token),
                    Err(err) => return Err(err),
                },
                _ => return Err(TokenizeError::UnexpectedToken),
            }
        }

        Ok(tokens)
    }

    fn tokenize_number(chars: &mut Peekable<Chars>) -> Result<TokenKind, TokenizeError> {
        let mut char_vec: Vec<char> = Vec::new();
        while let Some(&c) = chars.peek() {
            if c.is_ascii_digit() {
                chars.next();
                char_vec.push(c);
            } else {
                break;
            }
        }
        let number = char_vec.iter().collect::<String>().parse::<u32>().unwrap();

        Ok(TokenKind::Number(number))
    }
}
