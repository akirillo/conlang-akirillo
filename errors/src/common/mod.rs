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

/// This module contains a backtraced error and its methods.
pub mod backtraced;
pub use self::backtraced::*;

/// This module contains a formatted error and its methods.
pub mod formatted;
pub use self::formatted::*;

/// This module contains the macros for making errors easily.
#[macro_use]
pub mod macros;
pub use self::macros::*;

/// This module contains traits for making errors easily.
pub mod traits;
pub use self::traits::*;
