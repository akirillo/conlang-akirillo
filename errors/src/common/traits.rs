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

/// MessageCode trait that all Errors should implement.
pub trait ConlangMessageCode: Sized {
    /// Returns the error's exit code for the program.
    fn exit_code(&self) -> i32;

    /// Returns the prefixed error identifier.
    fn error_code(&self) -> String;

    /// Returns the prefixed warning identifier.
    fn warning_code(&self) -> String;

    /// Returns the messages's exit code mask, as to avoid conflicts.
    fn code_mask() -> i32;

    /// Returns the message's code type for the program.
    fn message_type() -> String;

    /// Returns if the message is an error or warning.
    fn is_error() -> bool;

    /// The ConlangErrorCode which has a default code identifier of 037
    /// (Conlang upsidedown and backwards). This is to make the exit codes
    /// unique to Conlang itself.
    #[inline(always)]
    fn code_identifier() -> i8 {
        37
    }
}
