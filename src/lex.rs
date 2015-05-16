use std::str;
use std::iter;

#[derive(Debug)]
pub enum Token {
    Unknown(char),
    OpenParen,
    CloseParen,
    Operator(char),
    Number(String),
}

pub struct Tokenizer<'a> {
    input: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer{
            input: input.chars().peekable(),
        }
    }

    // Reads a number from the input, appending characters to buf.
    fn read_number(&mut self, buf: &mut String) {
        // A dash can only appear as the first char of the number.
        let mut can_have_dash = buf.is_empty();

        loop {
            match self.input.peek() {
                Some(&c) if is_number(c) => {},
                Some(&c) if c == '.' => {},
                Some(&c) if can_have_dash && c == '-' => {},
                _ => break,
            };

            // At this point in the loop, we have always read at least one
            // char, so we can no longer accept dashes.
            can_have_dash = false;

            buf.push(self.input.next().unwrap());
        }
    }

    // Returns the token from the input, assuming that we already read a '-'.
    fn lex_dash(&mut self) -> Token {
        match self.input.peek() {
            Some(&c) if is_number(c) => {
                let mut buf = String::new();
                buf.push('-');
                self.read_number(&mut buf);
                Token::Number(buf)
            },
            _ => Token::Operator('-'),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let current = match self.input.next() {
                Some(c) => c,
                None => return None,
            };

            return match current {
                c if is_whitespace(c) => continue,
                '(' => Some(Token::OpenParen),
                ')' => Some(Token::CloseParen),
                '+' => Some(Token::Operator(current)),
                '*' => Some(Token::Operator(current)),
                '/' => Some(Token::Operator(current)),
                '-' => Some(self.lex_dash()),
                c if is_number(c) => {
                    let mut num = String::new();
                    num.push(current);
                    self.read_number(&mut num);
                    Some(Token::Number(num))
                },
                _ => {
                    Some(Token::Unknown(current))
                },
            };
        }
    }
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn is_number(c: char) -> bool {
    '0' <= c && c <= '9'
}
