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

//! The parser to convert Conlang code text into an [`Program`] AST type.
//!
//! This module contains the [`parse()`] method which calls the underlying [`tokenize()`]
//! method to create a new program ast.

use crate::Token;

use conlang_ast::*;
use conlang_errors::{emitter::Handler, Result};

use conlang_span::span::BytePos;

mod context;
use context::ParserContext;

mod input;

mod program;

/// Creates a new program from a given file path and source code text.
pub fn parse_program(handler: &Handler, source: &str, start_pos: BytePos) -> Result<Program> {
    // Tokenize the input.
    let tokens = crate::tokenize(source, start_pos)?;
    // Create a new parser context.
    let mut context = ParserContext::new(handler, tokens);
    // Parse the program.
    context.parse_program()
}

/// Parses an input file at the given file `path` and `source` code text.
pub fn parse_input(handler: &Handler, source: &str, start_pos: BytePos) -> Result<Assignment> {
    // Tokenize the input.
    let tokens = crate::tokenize(source, start_pos)?;
    // Create a new parser context.
    let mut context = ParserContext::new(handler, tokens);
    // Parse the input file.
    context.parse_input()
}
