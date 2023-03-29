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

use crate::ConlangMessageCode;

/// Contains the AST error definitions.
pub mod ast;
pub use self::ast::*;

/// Contains the CLI error definitions.
pub mod cli;
pub use self::cli::*;

/// Contains the Compiler error definitions.
pub mod compiler;
pub use self::compiler::*;

/// Contains the Input error definitions.
pub mod input;
pub use self::input::*;

/// Contains the Parser error definitions.
pub mod parser;
pub use self::parser::*;

/// Contains the Type Checker error definitions.
pub mod type_checker;
pub use self::type_checker::*;

/// The ConlangError type that contains all sub error types.
/// This allows a unified error type throughout the Conlang crates.
#[derive(Debug, Error)]
pub enum ConlangError {
    /// Represents an AST Error in a Conlang Error.
    #[error(transparent)]
    AstError(#[from] AstError),
    /// Represents an CLI Error in a Conlang Error.
    #[error(transparent)]
    CliError(#[from] CliError),
    /// Represents an Compiler Error in a Conlang Error.
    #[error(transparent)]
    CompilerError(#[from] CompilerError),
    /// Represents an Input Error in a Conlang Error.
    #[error(transparent)]
    InputError(#[from] InputError),
    /// Represents an Parser Error in a Conlang Error.
    #[error(transparent)]
    ParserError(#[from] ParserError),
    /// Represents a Type Checker Error in a Conlang Error.
    #[error(transparent)]
    TypeCheckerError(#[from] TypeCheckerError),
    /// Purely for just exiting with the correct status code and
    /// not re-displaying an error.
    #[error("")]
    LastErrorCode(i32),
    /// Anyhow errors.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl ConlangError {
    /// Implement error code for each type of Error.
    pub fn error_code(&self) -> String {
        use ConlangError::*;

        match self {
            AstError(error) => error.error_code(),
            CompilerError(error) => error.error_code(),
            CliError(error) => error.error_code(),
            InputError(error) => error.error_code(),
            ParserError(error) => error.error_code(),
            TypeCheckerError(error) => error.error_code(),
            LastErrorCode(_) => unreachable!(),
            Anyhow(_) => unimplemented!(), // todo: implement error codes for snarkvm errors.
        }
    }

    /// Implement exit code for each type of Error.
    pub fn exit_code(&self) -> i32 {
        use ConlangError::*;

        match self {
            AstError(error) => error.exit_code(),
            CompilerError(error) => error.exit_code(),
            CliError(error) => error.exit_code(),
            InputError(error) => error.exit_code(),
            ParserError(error) => error.exit_code(),
            TypeCheckerError(error) => error.exit_code(),
            LastErrorCode(code) => *code,
            Anyhow(_) => unimplemented!(), // todo: implement exit codes for snarkvm errors.
        }
    }
}

/// A global result type for all Conlang crates, that defaults the errors to be a ConlangError.
pub type Result<T, E = ConlangError> = core::result::Result<T, E>;
