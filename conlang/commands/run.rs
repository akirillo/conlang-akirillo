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

use crate::commands::Command;

use conlang_compiler::Compiler;
use conlang_errors::{emitter::Handler, Result};

use clap::StructOpt;
use std::path::PathBuf;

/// Build, Prove and Run Conlang program with inputs
#[derive(StructOpt, Debug)]
pub struct Run {
    #[structopt(name = "PATH", help = "The path to the `.conlang` file.")]
    program_path: PathBuf,

    #[structopt(name = "INPUT", help = "The path to a `.in` file.")]
    input_path: PathBuf,
}

impl Command for Run {
    type Input = ();
    type Output = ();

    fn prelude(&self) -> Result<Self::Input> {
        Ok(())
    }

    fn apply(self, _: Self::Input) -> Result<Self::Output> {
        // Initialize error handler
        let handler = Handler::default();

        // Compile the program.
        let mut compiler = Compiler::new(&handler, &self.program_path);
        compiler.compile()?;

        // Evaluate the input.
        match compiler.evaluate_input(&self.input_path)? {
            true => println!("The program is satisfied."),
            false => println!("The program is not satisfied."),
        }

        Ok(())
    }
}
