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

use super::*;

use crate::tokenizer::SpannedToken;
use conlang_errors::{ParserError, Result};

impl ParserContext<'_> {
    /// Returns an [`Assignment`] struct filled with the data acquired in the input file.
    #[allow(unused)]
    pub(crate) fn parse_input(&mut self) -> Result<Assignment> {
        let mut inputs = Vec::new();

        while self.has_next() {
            let SpannedToken { token, span } = self.peek();
            // If the next token is an identifier, then attempt to parse an input assignment.
            if matches!(token, Token::Identifier(_)) {
                // Parse the variable.
                let variable = self.parse_variable()?;
                // Parse the assignment operator.
                self.expect(&Token::Assign)?;
                // Parse the constant value.
                let constant = self.parse_constant()?;
                // Parse the semicolon.
                self.expect(&Token::Semicolon)?;
                // Add the input to the list.
                inputs.push((variable, constant));
            } else {
                return Err(ParserError::unexpected_token(token, *span).into());
            }
        }
        // Construct the assignment.
        Assignment::new(inputs)
    }

    /// Returns an [`Assignment`] struct filled with the data acquired in the input file.
    /// This method handles parser errors more gracefully.
    #[allow(unused)]
    pub(crate) fn parse_input_with_recovery(&mut self) -> Result<Assignment> {
        todo!()
    }
}
