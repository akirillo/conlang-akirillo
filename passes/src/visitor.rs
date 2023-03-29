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

/// A Visitor trait for traversing the AST.
pub trait Visitor<'a> {
    type AdditionalInput: Default;
    type Output: Default;

    /// Visit a program.
    fn visit_program(&mut self, program: &'a Program, additional_input: &Self::AdditionalInput) -> Self::Output {
        for constraint in &program.constraints {
            self.visit_constraint(constraint, additional_input);
        }
        Default::default()
    }

    /// Visit a constraint.
    fn visit_constraint(
        &mut self,
        constraint: &'a Constraint,
        additional_input: &Self::AdditionalInput,
    ) -> Self::Output {
        self.visit_linear_combination(&constraint.a, additional_input);
        self.visit_linear_combination(&constraint.b, additional_input);
        self.visit_linear_combination(&constraint.c, additional_input);
        Default::default()
    }

    /// Visit a linear combination.
    fn visit_linear_combination(
        &mut self,
        linear_combination: &'a LinearCombination,
        additional_input: &Self::AdditionalInput,
    ) -> Self::Output {
        for term in &linear_combination.terms {
            self.visit_term(term, additional_input);
        }
        self.visit_constant(&linear_combination.constant, additional_input);
        Default::default()
    }

    /// Visit a term.
    fn visit_term(&mut self, term: &'a Term, additional_input: &Self::AdditionalInput) -> Self::Output {
        self.visit_constant(&term.constant, additional_input);
        self.visit_variable(&term.variable, additional_input);
        Default::default()
    }

    /// Visit a variable.
    fn visit_variable(&mut self, _variable: &'a Variable, _additional_input: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    /// Visit a constant.
    fn visit_constant(&mut self, _constant: &'a Constant, _additional_input: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }
}
