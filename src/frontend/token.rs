use crate::utils::loc::Loc;

enum Literal {
    Integer,
    Float,
    Bool,
    Char,
    String
}

enum Keyword {
    Func,
    Return
}

enum Symbol {
    Plus,
    Minus,
    Times,
    LeftPar,
    RightPar
}

enum Type {
    Identifier,
    Literal(Literal),
    Keyword(Keyword),
    Symbol(Symbol),
    Newline
}

struct Token {
    ty: Type,
    value: String,
    loc: Loc
}
