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

use conlang_ast::{Assignment, Constraint, LinearCombination, Program, Term};
use conlang_errors::{CompilerError, Result};

use snarkvm_console::{network::Testnet3, types::Field};

pub struct Evaluator;

impl Evaluator {
    pub fn check_assignment(program: &Program, assignment: &Assignment) -> bool {
        let mut is_satisfied = true;
        for Constraint { a, b, c } in program.constraints.iter() {
            let a = Self::evaluate_linear_combination(a, assignment).unwrap();
            let b = Self::evaluate_linear_combination(b, assignment).unwrap();
            let c = Self::evaluate_linear_combination(c, assignment).unwrap();
            if a * b != c {
                is_satisfied = false;
            }
        }
        is_satisfied
    }

    fn evaluate_linear_combination(
        linear_combination: &LinearCombination,
        assignment: &Assignment,
    ) -> Result<Field<Testnet3>> {
        let mut result = linear_combination.constant.value;

        for Term { variable, constant } in linear_combination.terms.iter() {
            let value = match assignment.map.get(&variable.name) {
                Some(value) => value,
                None => return Err(CompilerError::variable_not_assigned(variable).into()),
            };
            result += value.value * constant.value;
        }

        Ok(result)
    }
}
