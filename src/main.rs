use crate::lexer::ToTokens;

mod lexer;
mod macros;

fn main() {
    let input = include_str!("../input.paip").trim();

    let tokens = input.char_indices().tokens();
    dbg!(input);
    tokens.for_each(|t| println!("{:?}", t));
    // TODO syntax iterator
}

mod syntax {

    #[derive(Debug, Clone, Hash)]
    pub enum Value<'a> {
        List(Vec<Expr<'a>>), // [ expr* ]
        This,                // @
        Name(&'a str),       // name
        Builtin(fn(Expr<'a>) -> Expr<'a>),
    }

    #[derive(Debug, Clone, Hash)]
    pub enum Expr<'a> {
        Val(Value<'a>),

        Let(Box<Expr<'a>>, Box<Expr<'a>>), // name: name
        Get(Box<Expr<'a>>, Box<Expr<'a>>), // this::name | name::name | path::name
        Set(Box<Expr<'a>>, Box<Expr<'a>>), // name = expr | deff = expr

        Run(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>), // expr .name list
        Ret(Box<Expr<'a>>),                               // @> expr
    }
}
pub mod runtime {
    use crate::syntax::Expr;
    use std::collections::HashMap;

    // Naive + types do nothing

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Value {
        Local(usize),
        Owned(usize),
    }

    #[derive(Debug, Default, Clone)]
    pub struct Ctx<'a> {
        pub names: HashMap<&'a str, Value>, // Reference exprs
        pub exprs: Vec<Expr<'a>>,
    }
}

pub trait PeekN: Iterator + Clone {
    fn peek_n(&self, n: usize) -> Option<Self::Item> {
        let mut tmp = (*self).clone();
        for _ in 0..n {
            tmp.next();
        }
        tmp.next()
    }
    fn skip_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
    fn skip_any(&mut self, p: fn(Self::Item) -> bool) {
        while let Some(e) = self.peek_n(0) {
            if p(e) {
                self.skip_n(1);
            }
        }
    }
}
impl<'a> PeekN for std::str::CharIndices<'a> {}
