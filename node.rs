use std::fmt;

use crate::token::Token;

pub enum PrefixOp {
    Neg,
    BitNot,
    LogicNot,
}

pub enum PostfixOp {
    Fac,
}

#[derive(Clone)]
pub enum UnaryOp {
    Neg,
    BitNot,
    LogicNot,

    Fac,
}

#[derive(Clone)]
pub enum BinaryOp {
    Add,
    Sub,

    Mul,
    Div,
    Mod,

    Pow,

    BitAnd,
    BitOr,
    BitXor,

    ShiftLeft,
    ShiftRight,
}

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(i64),
}

#[derive(Clone)]
pub enum Node {
    Val(Value),
    Var(String),
    Unary(UnaryOp, Box<Node>),
    Binary(BinaryOp, Box<Node>, Box<Node>),
}

impl From<&Token> for Option<PrefixOp> {
    fn from(i: &Token) -> Self {
        let res = match i {
            Token::Dash => PrefixOp::Neg,
            Token::Worm => PrefixOp::BitNot,
            Token::Exc => PrefixOp::LogicNot,

            _ => return None,
        };
        Some(res)
    }
}

impl From<&Token> for Option<PostfixOp> {
    fn from(i: &Token) -> Self {
        let res = match i {
            Token::Exc => PostfixOp::Fac,

            _ => return None,
        };
        Some(res)
    }
}

impl From<&PrefixOp> for UnaryOp {
    fn from(i: &PrefixOp) -> Self {
        match i {
            PrefixOp::Neg => UnaryOp::Neg,
            PrefixOp::BitNot => UnaryOp::BitNot,
            PrefixOp::LogicNot => UnaryOp::LogicNot,
        }
    }
}

impl From<&PostfixOp> for UnaryOp {
    fn from(i: &PostfixOp) -> Self {
        match i {
            PostfixOp::Fac => UnaryOp::Fac,
        }
    }
}

impl From<&Token> for Option<BinaryOp> {
    fn from(i: &Token) -> Self {
        let res = match i {
            Token::Plus => BinaryOp::Add,
            Token::Dash => BinaryOp::Sub,

            Token::Star => BinaryOp::Mul,
            Token::Slash => BinaryOp::Div,
            Token::Percent => BinaryOp::Mod,

            Token::StarStar => BinaryOp::Pow,

            Token::And => BinaryOp::BitAnd,
            Token::Pipe => BinaryOp::BitOr,
            Token::Hat => BinaryOp::BitXor,

            Token::LessLess => BinaryOp::ShiftLeft,
            Token::MoreMore => BinaryOp::ShiftRight,

            _ => return None,
        };
        Some(res)
    }
}

impl From<&Token> for Option<Value> {
    fn from(i: &Token) -> Self {
        let res = match i {
            Token::Num(n) => {
                let n = n.clone();
                Value::Num(n)
            }
            Token::Str(s) => {
                let s = s.clone();
                Value::Str(s)
            }
            _ => return None,
        };
        Some(res)
    }
}

impl PrefixOp {
    pub fn binding_power(&self) -> u8 {
        match self {
            PrefixOp::Neg => 130,
            PrefixOp::BitNot => 130,
            PrefixOp::LogicNot => 130,
        }
    }
}

impl PostfixOp {
    pub fn binding_power(&self) -> u8 {
        match self {
            PostfixOp::Fac => 70,
        }
    }
}

impl BinaryOp {
    pub fn binding_power(&self) -> (u8, u8) {
        match self {
            BinaryOp::BitOr => (80, 81),
            BinaryOp::BitAnd => (82, 83),
            BinaryOp::BitXor => (84, 85),

            BinaryOp::ShiftLeft |
            BinaryOp::ShiftRight => (110, 111),

            BinaryOp::Add | 
            BinaryOp::Sub => (120, 121),

            BinaryOp::Mul | 
            BinaryOp::Div | 
            BinaryOp::Mod => (122, 123),

            BinaryOp::Pow => (124, 125)
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            UnaryOp::Neg => "Neg",
            UnaryOp::BitNot => "BitNot",
            UnaryOp::LogicNot => "LogicNot",

            UnaryOp::Fac => "Fac",
        };
        write!(f, "{}", sym)
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sym = match self {
            BinaryOp::Add => "Add",
            BinaryOp::Sub => "Sub",

            BinaryOp::Mul => "Mul",
            BinaryOp::Div => "Div",
            BinaryOp::Mod => "Mod",

            BinaryOp::Pow => "Pow",

            BinaryOp::BitAnd => "BitAnd",
            BinaryOp::BitOr => "BitOr",
            BinaryOp::BitXor => "BitXor",

            BinaryOp::ShiftLeft => "ShiftLeft",
            BinaryOp::ShiftRight => "ShiftRight",
        };
        write!(f, "{}", sym)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Num(n) => return write!(f, "{}", n),
            Value::Str(s) => return write!(f, r#""{}""#, s),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Val(val) => write!(f, "{}", val),
            Node::Var(name) => write!(f, "{}", name),
            Node::Unary(op, node) => write!(f, "({} {})", op, node),
            Node::Binary(op, left, right) => write!(f, "({} {} {})", op, left, right),
        }
    }
}

impl BinaryOp {}

impl Node {
    pub fn eval(self) -> Node {
        match self {
            Node::Binary(op, left, right) => {
                let left_res = left.eval();
                let right_res = right.eval();
                match op {
                    BinaryOp::Add => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) =>  Node::Val(Value::Num(left_num + right_num)),
                        _ => panic!("not supported")
                    },
                    BinaryOp::Sub => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num - right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::Mul => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num * right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::Div => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) =>  Node::Val(Value::Num(left_num / right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::Mod => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num % right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::Pow => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num.pow(right_num as u32))),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::BitAnd => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num & right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::BitOr => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num | right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::BitXor => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num ^ right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::ShiftLeft => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num << right_num)),
                        _ => panic!("not supported"),
                    },
                    BinaryOp::ShiftRight => match (left_res, right_res) {
                        (
                            Node::Val(Value::Num(left_num)), 
                            Node::Val(Value::Num(right_num))
                        ) => Node::Val(Value::Num(left_num >> right_num)),
                        _ => panic!("not supported"),
                    },
                }
            },
            node => node,
        }
    }
}
