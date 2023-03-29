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
use std::{error::Error as ErrorArg, fmt::Debug};

create_messages!(
    /// CliError enum that represents all the errors for the  `conlang-lang` crate.
    CliError,
    code_mask: 7000i32,
    code_prefix: "CLI",

    /// For when the CLI experiences an IO error.
    @backtraced
    cli_io_error {
        args: (error: impl ErrorArg),
        msg: format!("cli io error {error}"),
        help: None,
    }

);
