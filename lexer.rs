use std::iter::{Iterator, Peekable};

use crate::token::Token;

static WHITES: &'static [char] = &[
    9 as char,
    32 as char
];
static LINES: &'static [char] = &[
    13 as char,
    10 as char
];

pub trait CharIterator: Iterator<Item=char> {}
impl <T: Iterator<Item=char>> CharIterator for T {}

#[derive(Clone)]
pub struct Lexer<T: CharIterator> {
    feed: Peekable<T>
}

impl <T: CharIterator> Lexer<T> {
    pub fn new(feed: T) -> Self {
        Self {
            feed: feed.peekable()
        }
    }
    fn skip_these(&mut self, chk: &[char]) {
        while let Some(c) = self.feed.peek() {
            if !chk.contains(c) {
                return
            }
            self.feed.next();
        }
    }
    fn expect(&mut self, c: char) -> bool {
        if let Some(&nc) = self.feed.peek() {
            if nc == c {
                self.feed.next();
                return true
            }
        }
        false
    }
    fn expect_these(&mut self, chk: &[char]) -> Option<char> {
        if let Some(&nc) = self.feed.peek() {
            if chk.contains(&nc) {
                self.feed.next();
                return Some(nc)
            }
        }
        None
    }
    fn expect_pred<F>(&mut self, f: F) -> Option<char> where F: Fn(&char) -> bool {
        if let Some(&nc) = self.feed.peek() {
            if f(&nc) {
                self.feed.next();
                return Some(nc)
            }
        }
        None
    }
}

fn is_name_first(c: &char) -> bool { c.is_ascii_alphabetic()   || *c == '_' }
fn is_name      (c: &char) -> bool { c.is_ascii_alphanumeric() || *c == '_' }

fn to_digit(c: char) -> i64 { let num = c as u8 - '0' as u8; num as i64 }

impl <T: CharIterator> Iterator for Lexer<T> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_these(WHITES);

        let tok = 
        if let Some(..) = self.expect_these(LINES) {
            self.skip_these(LINES);
            Token::Line
        }
        else if let Some(c) = self.expect_pred(is_name_first) {
            let mut s = String::from(c);
            while let Some(c) = self.expect_pred(is_name) {
                s.push(c)
            }
            Token::Name(s)
        }
        else if let Some(c) = self.expect_pred(char::is_ascii_digit) {
            let mut n = to_digit(c);
            while let Some(c) = self.expect_pred(char::is_ascii_digit) {
                n = n * 10 + to_digit(c);
            }
            Token::Num(n)
        }
        else if self.expect('+') { Token::Plus }
        else if self.expect('-') { Token::Dash }

        else if self.expect('*') {
            if self.expect('*') { Token::StarStar }
            else                { Token::Star }
        }
        else if self.expect('/') { Token::Slash }
        else if self.expect('%') { Token::Percent }

        else if self.expect('&') { Token::And }
        else if self.expect('|') { Token::Pipe }
        else if self.expect('^') { Token::Hat }
        else if self.expect('~') { Token::Worm }
        else if self.expect('!') { Token::Exc }

        else if self.expect('<') {
            if self.expect('<') { Token::LessLess }
            else                { todo!() }
        }
        else if self.expect('>') {
            if self.expect('>') { Token::MoreMore }
            else                { todo!() }
        }
        else if self.expect('(') { Token::OpenParen }
        else if self.expect(')') { Token::CloseParen }

        else if let Some(c) = self.feed.peek() {
            panic!("unknow character {}", c)
        }
        else { return None };

        Some(tok)
    }
}
