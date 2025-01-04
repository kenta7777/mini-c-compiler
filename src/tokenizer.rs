pub mod tokenizer {
    use std::{char, iter::Peekable, str::Chars};

    #[derive(Debug, PartialEq)]
    pub enum TokenKind {
        Plus,
        Minus,
        Number(u32),
    }

    #[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenizer::TokenizeError;
    use crate::{tokenize, TokenKind};

    #[test]
    fn test_tokenize_input_string_of_single_number() {
        let mut tokens = Vec::new();
        tokens.push(TokenKind::Number(123));

        assert_eq!(tokenize(&"123".to_string()), Ok(tokens));
    }

    #[test]
    fn test_tokenize_input_string_of_plus_and_minus_op() {
        let mut tokens = Vec::new();
        tokens.push(TokenKind::Number(123));
        tokens.push(TokenKind::Plus);
        tokens.push(TokenKind::Number(12));
        tokens.push(TokenKind::Minus);
        tokens.push(TokenKind::Number(1));

        assert_eq!(tokenize(&"123 + 12-1".to_string()), Ok(tokens));
    }

    #[test]
    fn test_tokenize_returns_error() {
        assert_eq!(
            tokenize(&"123 * 2".to_string()),
            Err(TokenizeError::UnexpectedToken)
        );
    }
}
