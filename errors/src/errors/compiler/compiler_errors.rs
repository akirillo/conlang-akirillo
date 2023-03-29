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

use crate::create_messages;

use std::{
    error::Error as ErrorArg,
    fmt::{Debug, Display},
};

create_messages!(
    /// CompilerError enum that represents all the errors for the  `conlang-compiler` crate.
    CompilerError,
    code_mask: 6000i32,
    code_prefix: "CMP",

    /// For when the compiler can't read a file from the provided path.
    @backtraced
    file_read_error {
        args: (path: impl Debug, error: impl ErrorArg),
        msg: format!("Cannot read from the provided file path '{path:?}': {error}"),
        help: None,
    }

    /// For when the evaluator cannot find a value for a variable during evaluation.
    @backtraced
    variable_not_assigned {
        args: (variable: impl Display),
        msg: format!("Cannot find assignment for `{variable}`."),
        help: None,
    }


);
