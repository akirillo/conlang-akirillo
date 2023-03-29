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

//! The compiler for Conlang programs.
//!
//! The [`Compiler`] type compiles Conlang programs into R1CS circuits.
use conlang_ast::Program;
use conlang_errors::{emitter::Handler, CompilerError, Result};
use conlang_span::{session_globals::with_session_globals, source_map::FileName};

use crate::Evaluator;
use std::{fs, path::PathBuf};

/// The primary entry point of the Conlang compiler.
#[derive(Clone)]
pub struct Compiler<'a> {
    /// The handler is used for error and warning emissions.
    handler: &'a Handler,
    /// The path to the conlang file.
    path: &'a PathBuf,
    /// The AST for the program.
    program: Program,
}

impl<'a> Compiler<'a> {
    /// Returns a new Conlang compiler.
    pub fn new(handler: &'a Handler, path: &'a PathBuf) -> Self {
        Self { handler, path, program: Program::default() }
    }

    /// Parses and stores a program file content from a string, constructs a syntax tree, and generates a program.
    pub fn parse_program_from_string(&mut self, program_string: &str, name: FileName) -> Result<()> {
        // Register the source (`program_string`) in the source map.
        let prg_sf = with_session_globals(|s| s.source_map.new_source(program_string, name));

        // Use the parser to construct the abstract syntax tree (ast).
        self.program = conlang_parser::parse_program(self.handler, &prg_sf.src, prg_sf.start_pos)?;

        // Emit any errors from the handler.
        self.handler.last_err().map_err(|e| *e)?;

        Ok(())
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let program_string = fs::read_to_string(self.path).map_err(|e| CompilerError::file_read_error(self.path, e))?;

        self.parse_program_from_string(&program_string, FileName::Real(self.path.clone()))
    }

    /// Parses the input file and checks that the assignment satisfies the program.
    pub fn evaluate_input(&mut self, input_path: &PathBuf) -> Result<bool> {
        // Load the input file.
        let input_string = fs::read_to_string(input_path).map_err(|e| CompilerError::file_read_error(input_path, e))?;

        // Register the source (`input_string`) in the source map.
        let input_sf =
            with_session_globals(|s| s.source_map.new_source(&input_string, FileName::Real(input_path.clone())));

        let assignment = conlang_parser::parse_input(self.handler, &input_sf.src, input_sf.start_pos)?;

        // Emit any errors from the handler.
        self.handler.last_err().map_err(|e| *e)?;

        // Check that the assignment satisfies the program.
        Ok(Evaluator::check_assignment(&self.program, &assignment))
    }

    /// Runs the compiler stages.
    pub fn compiler_stages(&mut self) -> Result<()> {
        Ok(())
    }

    /// Returns a compiled Conlang program.
    pub fn compile(&mut self) -> Result<()> {
        // Parse the program.
        self.parse_program()?;
        // Run the intermediate compiler stages.
        self.compiler_stages()?;
        Ok(())
    }
}
