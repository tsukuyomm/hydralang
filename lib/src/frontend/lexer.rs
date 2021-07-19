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

    #[regex("\\d+")]
    Int,

    #[regex("\\-?\\d+\\.\\d+")]
    Float,

    #[token("true")]
    #[token("false")]
    Boolean,

    // --------------- Extras ---------------------
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Undefined,
}
