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

use crate::tokenizer::Token;
use conlang_errors::{ParserError, Result};
use conlang_span::Span;

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    iter::{from_fn, Peekable},
};

/// Eat an identifier, that is, a string matching '[a-zA-Z][a-zA-Z\d_]*', if any.
fn eat_identifier(input: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    input.peek().filter(|c| c.is_ascii_alphabetic())?;
    Some(from_fn(|| input.next_if(|c| c.is_ascii_alphanumeric() || c == &'_')).collect())
}

/// Checks if a char is a Unicode Bidirectional Override code point
fn is_bidi_override(c: char) -> bool {
    let i = c as u32;
    (0x202A..=0x202E).contains(&i) || (0x2066..=0x2069).contains(&i)
}

/// Ensure that `string` contains no Unicode Bidirectional Override code points.
fn ensure_no_bidi_override(string: &str) -> Result<()> {
    if string.chars().any(is_bidi_override) {
        return Err(ParserError::lexer_bidi_override().into());
    }
    Ok(())
}

impl Token {
    /// Returns a tuple: [(integer length, integer token)] if an integer can be eaten, otherwise returns [`None`].
    /// An integer can be eaten if its bytes are at the front of the given `input` string.
    fn eat_integer(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<(usize, Token)> {
        if input.peek().is_none() {
            return Err(ParserError::lexer_empty_input().into());
        }

        let mut int = String::new();
        while let Some(c) = input.next_if(|c| c.is_ascii_digit()) {
            if c == '0' && matches!(input.peek(), Some('x')) {
                int.push(c);
                int.push(input.next().unwrap());
                return Err(ParserError::lexer_hex_number_provided(int).into());
            }

            int.push(c);
        }

        Ok((int.len(), Token::Integer(int)))
    }

    /// Returns a tuple: [(token length, token)] if the next token can be eaten, otherwise returns an error.
    /// The next token can be eaten if the bytes at the front of the given `input` string can be scanned into a token.
    pub(crate) fn eat(input: &str) -> Result<(usize, Token)> {
        if input.is_empty() {
            return Err(ParserError::lexer_empty_input().into());
        }

        let input_str = input;
        let mut input = input.chars().peekable();

        // Returns one token matching one character.
        let match_one = |input: &mut Peekable<_>, token| {
            input.next();
            Ok((1, token))
        };

        match *input.peek().ok_or_else(ParserError::lexer_empty_input)? {
            x if x.is_ascii_whitespace() => return match_one(&mut input, Token::WhiteSpace),
            x if x.is_ascii_digit() => return Self::eat_integer(&mut input),
            '(' => return match_one(&mut input, Token::LeftParen),
            ')' => return match_one(&mut input, Token::RightParen),
            '*' => return match_one(&mut input, Token::Mul),
            '+' => return match_one(&mut input, Token::Add),
            '-' => return match_one(&mut input, Token::Sub),
            '/' => {
                input.next();
                if input.next_if_eq(&'/').is_some() {
                    // Find the end of the comment line.
                    // This works because the code 10 of line feed cannot appear as a byte
                    // in middle of a multi-byte UTF-8 encoding of a character,
                    // because those bytes all have the high bit set to 1;
                    // in UTF-8, the byte 10 can only appear as the single-byte encoding of line feed.
                    let comment = match input_str.as_bytes().iter().position(|c| *c == b'\n') {
                        None => input_str,
                        Some(idx) => &input_str[..idx + 1],
                    };

                    ensure_no_bidi_override(comment)?;
                    return Ok((comment.len(), Token::CommentLine(comment.to_owned())));
                } else {
                    // Check that the next character is a block comment.
                    if input.next_if_eq(&'*').is_none() {
                        return Err(ParserError::could_not_lex_block_comment().into());
                    }
                    let mut comment = String::from("/*");

                    if input.peek().is_none() {
                        return Err(ParserError::lexer_empty_block_comment().into());
                    }

                    let mut ended = false;
                    while let Some(c) = input.next() {
                        comment.push(c);
                        if c == '*' && input.next_if_eq(&'/').is_some() {
                            comment.push('/');
                            ended = true;
                            break;
                        }
                    }

                    ensure_no_bidi_override(&comment)?;

                    if !ended {
                        return Err(ParserError::lexer_block_comment_does_not_close_before_eof(comment).into());
                    }
                    return Ok((comment.len(), Token::CommentBlock(comment)));
                }
            }
            ';' => return match_one(&mut input, Token::Semicolon),
            '=' => return match_one(&mut input, Token::Assign),
            _ => (),
        }
        if let Some(identifier) = eat_identifier(&mut input) {
            return Ok((identifier.len(), Token::Identifier(identifier)));
        }

        Err(ParserError::could_not_lex(input.take_while(|c| *c != ';' && !c.is_whitespace()).collect::<String>())
            .into())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    /// Returns a dummy token at a dummy span.
    pub const fn dummy() -> Self {
        Self { token: Token::Question, span: Span::dummy() }
    }
}

impl fmt::Display for SpannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}' @ ", self.token.to_string().trim())?;
        self.span.fmt(f)
    }
}

impl fmt::Debug for SpannedToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <SpannedToken as fmt::Display>::fmt(self, f)
    }
}
