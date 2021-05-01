use std::iter::Peekable;

use crate::token::Token;
use crate::node::{Node, Value, PrefixOp, PostfixOp, UnaryOp, BinaryOp};

use crate::lexer::{Lexer, CharIterator};

pub struct Parser<T: CharIterator> {
    feed: Peekable<Lexer<T>>
}

pub enum Result {
    None,
    Ok(Node),
    Err(String)
}

impl <T: CharIterator> Parser<T> {
    pub fn new(feed: Lexer<T>) -> Self {
        Self {
            feed: feed.peekable()
        }
    }
    pub fn parse(&mut self) -> Result {
        let node = self.parse_inner(0);
        if let Result::Err(err) = node {
            return Result::Err(err)
        }
        if let Some(tok) = self.feed.next() {
            let err = format!("parser expect end of input, found {}", tok);
            return Result::Err(err)
        }
        node
    }
    fn parse_inner(&mut self, power: u8) -> Result {
        let mut node = match self.feed.next() {
            None => return Result::None,
            Some(tok) => match tok {
                Token::Name(name) => Node::Var(name),
                tok if let Some(val) = Option::<Value>::from(&tok) => Node::Val(val),
                Token::OpenParen => match self.parse_inner(0) {
                    Result::None => {
                        let err = format!("open parentheses expect expression");
                        return Result::Err(err)
                    }
                    Result::Err(err) => return Result::Err(err),
                    Result::Ok(node) => match self.feed.next() {
                        Some(Token::CloseParen) => node,
                        Some(tok) => {
                            let err = format!("expect close parentheses, found {}", tok);
                            return Result::Err(err)
                        }
                        None => {
                            let err = format!("expect close parentheses");
                            return Result::Err(err)
                        }
                    }
                }
                tok if let Some(op) = Option::<PrefixOp>::from(&tok) => {
                    let right_power = op.binding_power();
                    match self.parse_inner(right_power) {
                        Result::Err(err) => return Result::Err(err),
                        Result::None => {
                            let err = format!("prefix operator expect expression");
                            return Result::Err(err)
                        }
                        Result::Ok(node) => Node::Unary(
                            UnaryOp::from(&op),
                            Box::new(node)
                        )
                    }
                }
                tok => {
                    let err = format!("expect prefix token, found {}", tok);
                    return Result::Err(err)
                }
            }
        };
        loop {
            let tok = match self.feed.peek() {
                None => break,
                Some(tok) => tok
            };
            if let Some(op) = Option::<PostfixOp>::from(tok) {
                let left_power = op.binding_power();
                if left_power < power { 
                    break 
                }
                self.feed.next();
                node = Node::Unary(
                    UnaryOp::from(&op),
                    Box::new(node)
                )
            }
            else if let Some(op) = Option::<BinaryOp>::from(tok) {
                let (left_power, right_power) = op.binding_power();
                if left_power < power { 
                    break 
                }
                self.feed.next();
                match self.parse_inner(right_power) {
                    Result::Err(err) => return Result::Err(err),
                    Result::None => {
                        let err = format!("infix operator expect expression");
                        return Result::Err(err)
                    }
                    Result::Ok(right) => node = Node::Binary(
                        op,
                        Box::new(node),
                        Box::new(right)
                    )
                }
            }
            else { break }
        }
        Result::Ok(node)
    }
}
