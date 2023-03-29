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

use conlang_ast::*;

/// A Reconstructor trait for reconstructing the AST.
pub trait Reconstructor {
    type AdditionalOutput: Default;

    /// Reconstruct a program.
    fn reconstruct_program(&mut self, program: Program) -> (Program, Self::AdditionalOutput) {
        (
            Program {
                constraints: program
                    .constraints
                    .into_iter()
                    .map(|constraint| self.reconstruct_constraint(constraint).0)
                    .collect(),
            },
            Default::default(),
        )
    }

    /// Reconstruct a constraint.
    fn reconstruct_constraint(&mut self, constraint: Constraint) -> (Constraint, Self::AdditionalOutput) {
        (
            Constraint {
                a: self.reconstruct_linear_combination(constraint.a).0,
                b: self.reconstruct_linear_combination(constraint.b).0,
                c: self.reconstruct_linear_combination(constraint.c).0,
            },
            Default::default(),
        )
    }

    /// Reconstruct a linear combination.
    fn reconstruct_linear_combination(
        &mut self,
        linear_combination: LinearCombination,
    ) -> (LinearCombination, Self::AdditionalOutput) {
        (
            LinearCombination {
                terms: linear_combination.terms.into_iter().map(|term| self.reconstruct_term(term).0).collect(),
                constant: self.reconstruct_constant(linear_combination.constant).0,
            },
            Default::default(),
        )
    }

    /// Reconstruct a term.
    fn reconstruct_term(&mut self, term: Term) -> (Term, Self::AdditionalOutput) {
        (
            Term {
                constant: self.reconstruct_constant(term.constant).0,
                variable: self.reconstruct_variable(term.variable).0,
            },
            Default::default(),
        )
    }

    /// Reconstruct a variable.
    fn reconstruct_variable(&mut self, variable: Variable) -> (Variable, Self::AdditionalOutput) {
        (variable, Default::default())
    }

    /// Reconstruct a constant.
    fn reconstruct_constant(&mut self, constant: Constant) -> (Constant, Self::AdditionalOutput) {
        (constant, Default::default())
    }
}
