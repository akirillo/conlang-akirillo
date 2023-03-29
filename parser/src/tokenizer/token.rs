// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the conlang library.

// The conlang library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The conlang library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the conlang library. If not, see <https://www.gnu.org/licenses/>.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents all valid Conlang syntax tokens.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    // Lexical Grammar
    // Literals
    CommentLine(String),
    CommentBlock(String),
    Identifier(String),
    Integer(String),
    WhiteSpace,

    // Symbols
    Add,
    Sub,
    Mul,
    Assign,
    LeftParen,
    RightParen,
    Semicolon,

    // Meta Tokens
    Eof,
    Question, // Dummy token.
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            CommentLine(s) => write!(f, "{s}"),
            CommentBlock(s) => write!(f, "{s}"),
            Identifier(s) => write!(f, "{s}"),
            Integer(s) => write!(f, "{s}"),
            WhiteSpace => write!(f, "whitespace"),

            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Assign => write!(f, "="),
            LeftParen => write!(f, "("),
            RightParen => write!(f, ")"),
            Semicolon => write!(f, ";"),

            Eof => write!(f, "<eof>"),
            Question => write!(f, "?"),
        }
    }
}

/// Describes delimiters of a token sequence.
#[allow(unused)]
#[derive(Copy, Clone)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,
}

impl Delimiter {
    /// Returns the open/close tokens that the delimiter corresponds to.
    pub fn open_close_pair(self) -> (Token, Token) {
        match self {
            Self::Parenthesis => (Token::LeftParen, Token::RightParen),
        }
    }
}
