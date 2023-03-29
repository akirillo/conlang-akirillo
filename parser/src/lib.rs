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

//! The parser to convert Conlang code text into a [`Program`] type.
//!
//! This module contains the [`parse_program()`] method to create a new program ast.

#![forbid(unsafe_code)]
#![allow(clippy::vec_init_then_push)]
#![doc = include_str!("../README.md")]

pub(crate) mod tokenizer;
pub(crate) use tokenizer::*;

pub mod parser;
pub use parser::*;
