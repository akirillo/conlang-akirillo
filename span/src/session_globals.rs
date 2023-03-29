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

use crate::source_map::SourceMap;

/// All the globals for a compiler sessions.
#[derive(Default)]
pub struct SessionGlobals {
    /// The source map used in the compiler.
    pub source_map: SourceMap,
}

scoped_tls::scoped_thread_local!(pub static SESSION_GLOBALS: SessionGlobals);

/// Creates the session globals and then runs the closure `f`.
#[inline]
pub fn create_session_if_not_set_then<R>(f: impl FnOnce(&SessionGlobals) -> R) -> R {
    if !SESSION_GLOBALS.is_set() {
        let sg = SessionGlobals::default();
        SESSION_GLOBALS.set(&sg, || SESSION_GLOBALS.with(f))
    } else {
        SESSION_GLOBALS.with(f)
    }
}

/// Gives access to read or modify the session globals in `f`.
#[inline]
pub fn with_session_globals<R>(f: impl FnOnce(&SessionGlobals) -> R) -> R {
    SESSION_GLOBALS.with(f)
}
