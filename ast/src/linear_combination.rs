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

use crate::{Constant, Term};

/// The [`LinearCombination`] data type represents a linear combination.
/// A linear combination is *sum* of terms and constants.
/// The following are all valid linear combinations:
/// - `1 + x`
/// - `0`
/// - `-x`
/// - `x - 15y`
/// - `-0 + 3z`
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LinearCombination {
    pub terms: Vec<Term>,
    pub constant: Constant,
}
