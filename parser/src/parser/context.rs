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

#![allow(unused)]

use crate::{tokenizer::*, Token};

use conlang_ast::*;
use conlang_errors::{emitter::Handler, ParserError, Result};
use conlang_span::Span;

use std::{fmt::Display, mem};

/// Stores a program in tokenized format plus additional context.
/// May be converted into a [`Program`] AST by parsing all tokens.
pub(crate) struct ParserContext<'a> {
    /// Handler used to side-channel emit errors from the parser.
    pub(crate) handler: &'a Handler,
    /// All un-bumped tokens.
    tokens: Vec<SpannedToken>,
    /// The current token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after a `p.bump()`, we'll have `p.token = '3'`.
    pub(crate) token: SpannedToken,
    /// The previous token, i.e., if `p.tokens = ['3', *, '4']`,
    /// then after two `p.bump()`s, we'll have `p.token = '*'` and `p.prev_token = '3'`.
    pub(crate) prev_token: SpannedToken,
}

/// Dummy span used to appease borrow checker.
const DUMMY_EOF: SpannedToken = SpannedToken { token: Token::Eof, span: Span::dummy() };

impl<'a> ParserContext<'a> {
    /// Returns a new [`ParserContext`] type given a vector of tokens.
    pub fn new(handler: &'a Handler, mut tokens: Vec<SpannedToken>) -> Self {
        // Strip out comments.
        tokens.retain(|x| !matches!(x.token, Token::CommentLine(_) | Token::CommentBlock(_)));
        // For performance we reverse so that we get cheap `.pop()`s.
        tokens.reverse();

        let token = SpannedToken::dummy();
        let mut p = Self { handler, prev_token: token.clone(), token, tokens };
        p.bump();
        p
    }

    /// Parser a [`Constant`], or errors.
    pub(super) fn parse_constant(&mut self) -> Result<Constant> {
        if let Token::Integer(value) = &self.token.token {
            let value = value.clone();
            let span = self.token.span;
            self.bump();
            Constant::new(value, span)
        } else {
            Err(ParserError::unexpected(&self.token.token, "integer literal", self.token.span).into())
        }
    }

    /// Parses a [`Variable`], or errors.
    pub(super) fn parse_variable(&mut self) -> Result<Variable> {
        self.eat_variable()
            .ok_or_else(|| ParserError::unexpected(&self.token.token, "identifier", self.token.span).into())
    }

    /// Skips over tokens until the next token is `tok` or `Eof`.
    pub(super) fn skip_until(&mut self, tok: &Token) {
        while !(self.check(tok) || self.check(&Token::Eof)) {
            self.bump();
        }
    }

    /// Advances the parser cursor by one token.
    ///
    /// So e.g., if we had `previous = A`, `current = B`, and `tokens = [C, D, E]`,
    /// then after `p.bump()`, the state will be `previous = B`, `current = C`, and `tokens = [D, E]`.
    pub(crate) fn bump(&mut self) {
        // Probably a bug (infinite loop), as the previous token was already EOF.
        if let Token::Eof = self.prev_token.token {
            panic!("attempted to bump the parser past EOF (may be stuck in a loop)");
        }

        // Extract next token, or `Eof` if there was none.
        let next_token = self.tokens.pop().unwrap_or(SpannedToken { token: Token::Eof, span: self.token.span });

        // Set the new token.
        self.prev_token = mem::replace(&mut self.token, next_token);
    }

    /// Checks whether the current token is `tok`.
    pub(super) fn check(&self, tok: &Token) -> bool {
        &self.token.token == tok
    }

    /// Checks whether the current token is a `Token::Int(_)`.
    pub(super) fn check_int(&self) -> bool {
        matches!(&self.token.token, Token::Integer(_))
    }

    /// Returns `true` if the next token is equal to the given token.
    /// Advances the parser to the next token.
    pub(super) fn eat(&mut self, token: &Token) -> bool {
        self.check(token).then(|| self.bump()).is_some()
    }

    /// Look-ahead `dist` tokens of `self.token` and get access to that token there.
    /// When `dist == 0` then the current token is looked at.
    pub(super) fn look_ahead<'s, R>(&'s self, dist: usize, looker: impl FnOnce(&'s SpannedToken) -> R) -> R {
        if dist == 0 {
            return looker(&self.token);
        }

        let idx = match self.tokens.len().checked_sub(dist) {
            None => return looker(&DUMMY_EOF),
            Some(idx) => idx,
        };

        looker(self.tokens.get(idx).unwrap_or(&DUMMY_EOF))
    }

    /// Peek at the next token.
    pub(super) fn peek(&self) -> &SpannedToken {
        &self.token
    }

    /// Emit the error `err`.
    pub(super) fn emit_err(&self, err: ParserError) {
        self.handler.emit_err(err);
    }

    /// Returns true if the next token exists.
    pub(crate) fn has_next(&self) -> bool {
        !matches!(self.token.token, Token::Eof)
    }

    /// At the previous token, return and make a variable with `name`.
    fn mk_ident_prev(&self, name: String) -> Variable {
        Variable { name, span: self.prev_token.span }
    }

    /// Eats the next token if it is a variable and returns it.
    pub(super) fn eat_variable(&mut self) -> Option<Variable> {
        if let Token::Identifier(name) = self.token.token.clone() {
            self.bump();
            return Some(self.mk_ident_prev(name));
        }
        None
    }

    /// Eats any of the given `tokens`, returning `true` if anything was eaten.
    pub(super) fn eat_any(&mut self, tokens: &[Token]) -> bool {
        tokens.iter().any(|x| self.check(x)).then(|| self.bump()).is_some()
    }

    /// Returns an unexpected error at the current token.
    pub(super) fn unexpected<T>(&self, expected: impl Display) -> Result<T> {
        Err(ParserError::unexpected(&self.token.token, expected, self.token.span).into())
    }

    /// Eats the expected `token`, or errors.
    pub(super) fn expect(&mut self, token: &Token) -> Result<Span> {
        if self.eat(token) { Ok(self.prev_token.span) } else { self.unexpected(token) }
    }

    /// Eats one of the expected `tokens`, or errors.
    pub(super) fn expect_any(&mut self, tokens: &[Token]) -> Result<Span> {
        if self.eat_any(tokens) {
            Ok(self.prev_token.span)
        } else {
            self.unexpected(tokens.iter().map(|x| format!("'{x}'")).collect::<Vec<_>>().join(", "))
        }
    }

    /// Parses a list of `T`s using `inner`
    /// The opening and closing delimiters are `bra` and `ket`,
    /// and elements in the list are optionally separated by `sep`.
    /// When `(list, true)` is returned, `sep` was a terminator.
    pub(super) fn parse_list<T>(
        &mut self,
        delimiter: Delimiter,
        sep: Option<Token>,
        mut inner: impl FnMut(&mut Self) -> Result<Option<T>>,
    ) -> Result<(Vec<T>, bool, Span)> {
        let (open, close) = delimiter.open_close_pair();
        let mut list = Vec::new();
        let mut trailing = false;

        // Parse opening delimiter.
        let open_span = self.expect(&open)?;

        while !self.check(&close) {
            // Parse the element. We allow inner parser recovery through the `Option`.
            if let Some(elem) = inner(self)? {
                list.push(elem);
            }
            // Parse the separator, if any.
            if sep.as_ref().filter(|sep| !self.eat(sep)).is_some() {
                trailing = false;
                break;
            }

            trailing = true;
        }

        // Parse closing delimiter.
        let span = open_span + self.expect(&close)?;

        Ok((list, trailing, span))
    }

    /// Returns true if the current token is `(`.
    pub(super) fn peek_is_left_par(&self) -> bool {
        matches!(self.token.token, Token::LeftParen)
    }
}
