use std::{ops::Index, rc::Rc};

use crate::lexer::ToTokens;

mod lexer;
mod macros;

fn main() {
    let input = include_str!("../input.paip");

    let mut tokens = input.char_indices().tokens();
    dbg!(input);
    tokens.count();
    tokens
        .tokens
        .into_iter()
        .zip(
            tokens
                .positions
                .into_iter()
                .chain([input.len() as u32].into_iter())
                .skip(1),
        )
        .fold(0u32, |start, (t, end)| {
            let (start, end): (usize, usize) = (start as _, end as _);
            println!("'{}' - {t:?}", &input[start..end]);

            end as _
        });
}

// text -> tokens -> expr 󱞳

mod syntax {

    #[derive(Debug, Clone, Hash)]
    pub enum Expr<'a> {
        List(Vec<Expr<'a>>), // [ expr* ]
        This,                // @
        Name(&'a str),       // name
        Builtin(fn(Expr<'a>) -> Expr<'a>),

        Let(Box<Expr<'a>>, Box<Expr<'a>>), // name: name
        Get(Box<Expr<'a>>, Box<Expr<'a>>), // this::name | name::name | path::name
        Set(Box<Expr<'a>>, Box<Expr<'a>>), // name = expr | deff = expr

        Run(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>), // expr .name list
        Ret(Box<Expr<'a>>),                               // @> expr
    }

    // Can all be build from left to right
    // what happens with errors?
    // can we treat tokens as simple operators?
    //
    // All expr are either a list or name op expr
    //
    // [ open a new block
    // ] close the last block
    // @ this block
    // name select entry
    // : if next is name create new entry selected
    // ::
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
