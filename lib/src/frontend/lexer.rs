// #[regex]
//===------------- lexer.rs - The file for lexical analysis, tokens ----------===//
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

#[derive(Logos, Debug, PartialEq, Eq)]
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

    #[regex("[0-9]+")]
    Int,

    #[regex("\\-?\\d+\\.\\d+")]
    Float,

    #[token("true")]
    #[token("false")]
    Boolean,

    // ------------------- Symbols -------------------
    // ===---------- Math Operators ----------------===
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterics,

    #[token("/")]
    Slash,

    //===------------- Logical operators -----------===
    #[token("=")]
    Equal,

    #[token("!")]
    Exclamation,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("==")]
    IsEqual,

    #[token("!=")]
    NotEqual,

    #[token("<=")]
    LesserEqual,

    #[token(">=")]
    GreaterEqal,

    #[token("_")]
    Under,

    // Brackets
    #[token("<")]
    LesserThan,

    #[token(">")]
    GreaterThan,

    // --------------- Extras ---------------------
    #[regex(r"[ \t\n\f]+", logos::skip)]
    WhiteSpace,

    #[error]
    Error,
}
