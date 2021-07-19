// #[regex]
//===----------------- lexer.rs - The file for lexical analysis --------------===//
//
// This source file is part of the hydralang open souce project
//
// Copyright (c) 2021 KittyBorgX and the englang project authors
// Licensed under Apache License v2.0 with Runtime Library Exception
//
// See https://github.com/KittyBorgX/hydralang/blob/main/LICENSE for license information
// See https://github.com/KittyBorgX/hydralang/blob/main/CONTRIBUTORS.md for the list of hydralang project authors
//
//===----------------------------------------------------------------------===//

use logos::{Lexer, Logos, Span};
pub struct Token {
    span: std::ops::Range<usize>,
    kind: TokenKind,
}

#[derive(Logos, Debug, PartialEq)]
pub enum TokenKind {
    // -------------- Brackets ------------------
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftCurly,

    #[token("}")]
    RightCurly,

    #[token("\"")]
    Quote,
    // ------------- Data types ------------------
    #[regex("[a-zA-Z]+")]
    Str,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    // #[regex("[0-9]")]
    // #[regex("[0-9]")]
    Int(u64),

    #[token("a")]
    #[token("A")]
    #[token("b")]
    #[token("B")]
    #[token("c")]
    #[token("C")]
    #[token("d")]
    #[token("D")]
    #[token("e")]
    #[token("E")]
    #[token("f")]
    #[token("F")]
    #[token("g")]
    #[token("G")]
    #[token("h")]
    #[token("H")]
    #[token("i")]
    #[token("I")]
    #[token("j")]
    #[token("J")]
    #[token("k")]
    #[token("K")]
    #[token("l")]
    #[token("L")]
    #[token("m")]
    #[token("M")]
    #[token("n")]
    #[token("N")]
    #[token("o")]
    #[token("O")]
    #[token("p")]
    #[token("P")]
    #[token("q")]
    #[token("Q")]
    #[token("r")]
    #[token("R")]
    #[token("s")]
    #[token("S")]
    #[token("t")]
    #[token("T")]
    #[token("u")]
    #[token("U")]
    #[token("v")]
    #[token("V")]
    #[token("w")]
    #[token("W")]
    #[token("x")]
    #[token("X")]
    #[token("y")]
    #[token("Y")]
    #[token("z")]
    #[token("Z")]
    Char,

    #[regex("\\-?\\d+\\.\\d+")]
    Float,
    //
    //     #[regex(r#"'([^\\']|\\[nrt'"0])'"#, |lex| convert_chars(lex.slice(), 1).bytes().next().unwrap())]
    //     #[regex("\\d")]
    //     Char,
    //
    #[regex("_[^_c]")]
    #[token("true")]
    #[token("false")]
    Boolean,

    // --------------- Extras ---------------------
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Undefined,
}
