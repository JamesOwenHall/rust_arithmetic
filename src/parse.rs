use std::iter;

use lex;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug)]
pub enum Node {
    Number(f64),
    Operation{operator: char, children: Vec<Node>},
}

pub fn parse(input: &str) -> Result<Node, ParseError> {
    let mut parser = Parser{
        tokenizer: lex::Tokenizer::new(input).peekable(),
    };
    parser.parse_node()
}

struct Parser<'a> {
    tokenizer: iter::Peekable<lex::Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    fn parse_node(&mut self) -> Result<Node, ParseError> {
        let token = match self.tokenizer.next() {
            None => return Err(ParseError),
            Some(t) => t,
        };

        match token {
            lex::Token::Number(c) => {
                match c.parse::<f64>() {
                    Ok(f) => Ok(Node::Number(f)),
                    Err(_) => Err(ParseError)
                }
            },
            lex::Token::OpenParen => self.parse_operation(),
            _ => Err(ParseError),
        }
    }

    fn parse_operation(&mut self) -> Result<Node, ParseError> {
        let op = match self.tokenizer.next() {
            Some(lex::Token::Operator(t)) => t,
            _ => return Err(ParseError),
        };

        let mut children = vec![];
        
        loop {
            match self.tokenizer.peek() {
                Some(&lex::Token::CloseParen) => {
                    self.tokenizer.next();
                    break;
                },
                None => return Err(ParseError),
                _ => {
                    let child = try!(self.parse_node());
                    children.push(child);
                },
            };
        }

        Ok(Node::Operation{operator: op, children: children})
    }
}
