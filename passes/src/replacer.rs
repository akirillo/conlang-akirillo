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

use crate::Reconstructor;
use conlang_ast::Variable;

/// A `Replacer` applies `replacer` to all `Variables`s in an AST.
///
/// `Replacer`s can be used to rename variable.
///  To do so,
///    1. Create a `Replacer` with a closure that returns a new `Variable`.
///    2. Call `reconstruct` on the `Replacer` with the AST to be renamed.
///  For example,
/// ```rust,no_run
///     use conlang_ast::{Program, Variable};
///     use conlang_passes::Replacer;
///
///     let replacer = Replacer::new(|variable| {
///        Variable::new(format!("{}_renamed", variable.name))
///    });
///    let program = Program::default();
///    let renamed_program = replacer.reconstruct(program);
///  ```
pub struct Replacer<F>
where
    F: Fn(&Variable) -> Variable,
{
    replace: F,
}

impl<F> Replacer<F>
where
    F: Fn(&Variable) -> Variable,
{
    pub fn new(replace: F) -> Self {
        Self { replace }
    }
}

impl<F> Reconstructor for Replacer<F>
where
    F: Fn(&Variable) -> Variable,
{
    type AdditionalOutput = ();

    fn reconstruct_variable(&mut self, variable: Variable) -> (Variable, Self::AdditionalOutput) {
        ((self.replace)(&variable), Default::default())
    }
}
