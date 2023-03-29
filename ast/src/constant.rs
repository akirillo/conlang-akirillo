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

use conlang_errors::{ParserError, Result};
use conlang_span::Span;

use snarkvm_console::{network::Testnet3, prelude::Zero, types::Field};

use snarkvm_console::prelude::One;
use std::str::FromStr;

/// The [`Constant`] data type represents a constant in a linear combination.
/// A constant can either be standalone or part of a term.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Constant {
    pub value: Field<Testnet3>,
    pub repr: String,
    pub span: Span,
}

impl Constant {
    /// Returns a new constant from the given string representation.
    pub fn new(string: String, span: Span) -> Result<Self> {
        let value = Field::<Testnet3>::from_str(&format!("{string}field"))
            .map_err(|_| ParserError::constant_cannot_be_parsed_into_field_element(string, span))?;
        Ok(Self { value, repr: value.to_string(), span })
    }

    /// Negates the constant.
    pub fn negate(&mut self) {
        self.value = -self.value;
        self.repr = format!("-{}", self.repr);
    }

    /// Return the one constant.
    pub fn one() -> Self {
        Self { value: Field::<Testnet3>::one(), repr: "1".to_string(), span: Span::default() }
    }
}

impl Default for Constant {
    fn default() -> Self {
        Self { value: Field::<Testnet3>::zero(), repr: "0".to_string(), span: Span::default() }
    }
}
