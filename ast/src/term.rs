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

use crate::{Constant, Variable};

/// The [`Variable`] data type represents the product of a constant and variable in a linear combination.
/// The following are all valid terms:
/// - `1 * x`
/// - `0 * y`
/// - `-1 * z`
/// - `15 * x`
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Term {
    pub constant: Constant,
    pub variable: Variable,
}

impl Term {
    /// Negates the term.
    pub fn negate(&mut self) {
        self.constant.negate();
    }
}
