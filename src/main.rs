// use pest::Parser;
// use pest_derive::Parser;

// #[derive(Parser)]
// #[grammar = "./paip.pest"]
// struct PaipParser;

fn main() {
    // let input = include_str!("../input.paip").trim();
    // PaipParser::parse(todo!(), input);

    let input = "name: type = [  \n     something.func [ @::a=3 ] ]";
    let tokens = input.char_indices().tokens();
    dbg!(input);
    tokens.for_each(|t| println!("{:?}", t));
    // TODO syntax iterator
}

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Err,        // An unrecognized token
    Whitespace, // space
    Name,       // \w+
    Dot,        // .
    Func,       // >
    Sep,        // ;
    Eq,         // =
    Path,       // ::
    Type,       // :
    Ret,        // @>
    Ctx,        // @
    OpnBlk,     // [
    ClsBlk,     // ]
    OpnParens,  // (
    ClsParens,  // )
    EOI,        // End Of Input
}

#[derive(Debug, Clone)]
pub struct Tokens<T>
where
    T: PeekN<Item = (usize, char)>,
{
    iter: T,

    tokens: Vec<Kind>,
    positions: Vec<u32>,
}

impl<T> Tokens<T>
where
    T: PeekN<Item = (usize, char)>,
{
    fn add(&mut self, kind: Kind, start: u32) {
        self.tokens.push(kind);
        self.positions.push(start);
    }
}

macro_rules! build_parser {
    (
        singles: [ $($single_char: literal => $single_kind: expr),+  $(,)?],
        doubles: [
            $($start_char: literal => [
                $($last_char: literal => $double_kind: expr),+,
                else => $double_else: expr
            ]),+
        ],
        strings: [ $( $func:path => $func_kind:expr ),+ ]

    ) => {
    impl<T> Iterator for Tokens<T>
    where T: PeekN<Item=(usize, char)>
    {
        type Item = Kind;

        fn next(&mut self) -> Option<Self::Item> {
            let (i, c) = self.iter.peek_n(0)?;
            let tok = match c {
                $( $single_char => {
                        self.iter.skip_n(1);
                        $single_kind
                    } ),+
                $( $start_char => match self.iter.peek_n(1) {
                    $( Some((_, $last_char)) => {
                            self.iter.skip_n(2);
                            $double_kind
                        } ),+
                        None | Some(_) => {
                            self.iter.skip_n(1);
                            $double_else
                        }
                    } ),+
                $( e if $func(e) => {
                        while let Some((_, c)) = self.iter.peek_n(0)
                            && $func(c)
                        {
                            self.iter.skip_n(1);
                        }
                        $func_kind
                    } ),+

                    _ => {
                        self.iter.skip_n(1);
                        Kind::Err
                    }, // TODO maybe we can do something better with errors, other than treat them as tokens, specially once we start with the syntax part
                };
            self.add(tok, i as _);
            Some(tok)
            }
        }
    };

}

build_parser!(
    singles: [
        '.' => Kind::Dot,
        '>' => Kind::Func,
        ';' => Kind::Sep,
        '=' => Kind::Eq,
        '[' => Kind::OpnBlk,
        ']' => Kind::ClsBlk,
        '(' => Kind::OpnParens,
        ')' => Kind::ClsParens
    ],
    doubles: [
        ':' => [
            ':' => Kind::Path,
            else=> Kind::Type
        ],
        '@' => [
            '>' => Kind::Ret,
            else=> Kind::Ctx
        ]
    ],
    strings: [
        char::is_space => Kind::Whitespace,
        char::is_name => Kind::Name
    ]
);

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
impl<'a> PeekN for std::str::Chars<'a> {}

pub trait LexingUtils {
    fn is_name(self) -> bool;
    fn is_space(self) -> bool;
}

impl LexingUtils for char {
    fn is_name(self) -> bool {
        self.is_alphanumeric() || self == '_'
    }
    fn is_space(self) -> bool {
        self.is_whitespace()
    }
}

pub trait ToTokens<T>
where
    T: PeekN<Item = (usize, char)>,
{
    fn tokens(self) -> Tokens<T>;
}

impl<T> ToTokens<T> for T
where
    T: PeekN<Item = (usize, char)>,
{
    fn tokens(self) -> Tokens<T> {
        Tokens {
            iter: self,
            tokens: vec![],
            positions: vec![],
        }
    }
}
