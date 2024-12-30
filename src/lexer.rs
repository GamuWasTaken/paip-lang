use crate::{PeekN, build_parser};

build_parser!(
    singles: [
        '.' => Dot,
        '>' => Func,
        ';' => Sep,
        '=' => Eq,
        '[' => OpnBlk,
        ']' => ClsBlk,
        '(' => OpnParens,
        ')' => ClsParens
    ],
    doubles: [
        ':' => [
            ':' => Path,
            else=> Type
        ],
        '@' => [
            '>' => Ret,
            else=> Ctx
        ],
    ],
    strings: [
        char::is_space => Whitespace,
        char::is_name => Name
    ]
);

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
