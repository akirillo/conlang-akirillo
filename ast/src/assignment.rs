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

use conlang_errors::{InputError, Result};

use indexmap::IndexMap;

/// The [`Assignment`] data type represents an input assignment to a conlang program.
#[derive(Clone, Debug, Default)]
pub struct Assignment {
    pub map: IndexMap<String, Constant>,
}

impl Assignment {
    /// Returns a new [`Assignment`].
    /// This function errors if the variables are not unique.
    pub fn new(inputs: Vec<(Variable, Constant)>) -> Result<Self> {
        let mut assignment = Assignment::default();
        // Add each input to the assignment, checking for duplicates.
        for (variable, constant) in inputs {
            let span = variable.span;
            if assignment.map.insert(variable.name, constant).is_some() {
                return Err(InputError::duplicate_input_variable(span).into());
            }
        }
        Ok(assignment)
    }
}
