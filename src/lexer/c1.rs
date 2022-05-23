use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    #[error]
    #[regex(r"/\*[^\*/]*\*/", logos::skip)]
    #[regex("//[^\n]*", logos::skip)]
	#[regex(r"[ \t\n\f\r]+", logos::skip)]
	Error,
    // TODO: Define variants and their token/regex

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[regex("[0-9]+")]
    ConstInt,

    #[regex(r"[0-9]+\.[0-9]+")]
    #[regex(r"\.[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+e[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+E[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+e\+[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+e-[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+E\+[0-9]+")]
    #[regex(r"[0-9]+\.[0-9]+E-[0-9]+")]
    #[regex(r"\.[0-9]+e[0-9]+")]
    #[regex(r"\.[0-9]+E[0-9]+")]
    #[regex(r"\.[0-9]+e\+[0-9]+")]
    #[regex(r"\.[0-9]+e-[0-9]+")]
    #[regex(r"\.[0-9]+E\+[0-9]+")]
    #[regex(r"\.[0-9]+E-[0-9]+")]
    #[regex("[0-9]+e[0-9]+")]
    #[regex("[0-9]+E[0-9]+")]
    #[regex("[0-9]+e-[0-9]+")]
    #[regex(r"[0-9]+e\+[0-9]+")]
    #[regex("[0-9]+E-[0-9]+")]
    #[regex(r"[0-9]+E\+[0-9]+")]
    ConstFloat,

    #[token("true")]
    #[token("false")]
    ConstBoolean,

    #[regex("\"[^\"]*\"")] 
    ConstString,

    #[regex("[a-zA-Z]+[0-9]*", priority =3)]
    #[regex("[a-zA-Z]+[a-zA-Z]*")] 
    Id,
}
