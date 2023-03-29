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

//! The tokenizer to convert Conlang code text into tokens.
//!
//! This module contains the [`tokenize()`] method which breaks down string text into tokens,
//! optionally separated by whitespace.

pub(crate) mod token;

pub(crate) use self::token::*;

pub(crate) mod lexer;
pub(crate) use self::lexer::*;

use conlang_errors::Result;
use conlang_span::span::{BytePos, Pos, Span};
use std::iter;

/// Creates a new vector of spanned tokens from a given file path and source code text.
pub(crate) fn tokenize(input: &str, start_pos: BytePos) -> Result<Vec<SpannedToken>> {
    tokenize_iter(input, start_pos).collect()
}

/// Yields spanned tokens from the given source code text.
///
/// The `lo` byte position determines where spans will start.
pub(crate) fn tokenize_iter(mut input: &str, mut lo: BytePos) -> impl '_ + Iterator<Item = Result<SpannedToken>> {
    iter::from_fn(move || {
        while !input.is_empty() {
            let (token_len, token) = match Token::eat(input) {
                Err(e) => return Some(Err(e)),
                Ok(t) => t,
            };
            input = &input[token_len..];

            let span = Span::new(lo, lo + BytePos::from_usize(token_len));
            lo = span.hi;

            match token {
                Token::WhiteSpace => continue,
                _ => return Some(Ok(SpannedToken { token, span })),
            }
        }

        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use conlang_span::{session_globals::create_session_if_not_set_then, source_map::FileName};
    use std::fmt::Write;

    #[test]
    fn test_tokenizer() {
        create_session_if_not_set_then(|s| {
            let raw = r#"
    "test"
    "test{}test"
    "test{}"
    "{}test"
    "test{"
    "test}"
    "test{test"
    "test}test"
    "te{{}}"
    test_ident
    12345
    address
    assert
    assert_eq
    assert_neq
    async
    bool
    const
    else
    false
    field
    finalize
    for
    function
    group
    i128
    i64
    i32
    i16
    i8
    if
    in
    inline
    input
    let
    mut
    private
    program
    public
    return
    scalar
    self
    string
    struct
    test
    then
    transition
    true
    u128
    u64
    u32
    u16
    u8
    console
    !
    !=
    &&
    (
    )
    *
    **
    +
    ,
    -
    ->
    =>
    _
    .
    ..
    /
    :
    ;
    <
    <=
    =
    ==
    >
    >=
    [
    ]
    {{
    }}
    ||
    ?
    @
    // test
    /* test */
    //"#;
            let sf = s.source_map.new_source(raw, FileName::Custom("test".into()));
            let tokens = tokenize(&sf.src, sf.start_pos).unwrap();
            let mut output = String::new();
            for SpannedToken { token, .. } in tokens.iter() {
                write!(output, "{token} ").expect("failed to write string");
            }

            assert_eq!(
                output,
                r#""test" "test{}test" "test{}" "{}test" "test{" "test}" "test{test" "test}test" "te{{}}" test_ident 12345 address assert assert_eq assert_neq async bool const else false field finalize for function group i128 i64 i32 i16 i8 if in inline input let mut private program public return scalar self string struct test then transition true u128 u64 u32 u16 u8 console ! != && ( ) * ** + , - -> => _ . .. / : ; < <= = == > >= [ ] { { } } || ? @ // test
 /* test */ // "#
            );
        });
    }

    #[test]
    fn test_spans() {
        create_session_if_not_set_then(|s| {
            let raw = r#"
ppp            test
            // test
            test
            /* test */
            test
            /* test
            test */
            test
            "#;

            let sm = &s.source_map;
            let sf = sm.new_source(raw, FileName::Custom("test".into()));
            let tokens = tokenize(&sf.src, sf.start_pos).unwrap();
            let mut line_indicies = vec![0];
            for (i, c) in raw.chars().enumerate() {
                if c == '\n' {
                    line_indicies.push(i + 1);
                }
            }
            for token in tokens.iter() {
                assert_eq!(token.token.to_string(), sm.contents_of_span(token.span).unwrap());
            }
        })
    }
}
