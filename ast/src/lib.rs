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

//! The abstract syntax tree (AST) for a conlang program.

pub mod constant;
pub use constant::*;

pub mod constraint;
pub use constraint::*;

pub mod assignment;
pub use assignment::*;

pub mod linear_combination;
pub use linear_combination::*;

pub mod program;
pub use program::*;

pub mod term;
pub use term::*;

pub mod variable;
pub use variable::*;
