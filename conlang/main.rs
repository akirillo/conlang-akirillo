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

pub mod commands;

use crate::commands::*;
use conlang_errors::Result;
use conlang_span::session_globals::create_session_if_not_set_then;

use clap::StructOpt;
use std::{path::PathBuf, process::exit};

/// CLI Arguments entry point - includes global parameters and subcommands
#[derive(StructOpt, Debug)]
#[structopt(name = "conlang", author = "The Aleo Team <hello@aleo.org>")]
pub struct CLI {
    #[structopt(short, global = true, help = "Print additional information for debugging")]
    debug: bool,

    #[structopt(short, global = true, help = "Suppress CLI output")]
    quiet: bool,

    #[structopt(subcommand)]
    command: Commands,

    #[structopt(help = "Custom Aleo PM backend URL", env = "APM_URL")]
    api: Option<String>,

    #[structopt(long, global = true, help = "Optional path to Conlang program root folder", parse(from_os_str))]
    path: Option<PathBuf>,
}

///Conlang compiler and package manager
#[derive(StructOpt, Debug)]
enum Commands {
    #[structopt(about = "Run a program with input variables")]
    Run {
        #[structopt(flatten)]
        command: Run,
    },
}

fn set_panic_hook() {
    #[cfg(not(debug_assertions))]
    std::panic::set_hook({
        Box::new(move |e| {
            eprintln!("thread `{}` {}", std::thread::current().name().unwrap_or("<unnamed>"), e);
            eprintln!("stack backtrace: \n{:?}", backtrace::Backtrace::new());
            eprintln!("error: internal compiler error: unexpected panic\n");
            eprintln!("note: the compiler unexpectedly panicked. this is a bug.\n");
            eprintln!(
                "note: {} {} running on {} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                sys_info::os_type().unwrap_or_else(|e| e.to_string()),
                sys_info::os_release().unwrap_or_else(|e| e.to_string()),
            );
            eprintln!("note: compiler args: {}\n", std::env::args().collect::<Vec<_>>().join(" "));
            eprintln!("note: compiler flags: {:?}\n", CLI::parse());
        })
    });
}

pub fn handle_error<T>(res: Result<T>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("{err}");
            exit(err.exit_code());
        }
    }
}

/// Run command with custom build arguments.
pub fn run_with_args(cli: CLI) -> Result<()> {
    match cli.command {
        Commands::Run { command } => command.try_execute(),
    }
}

fn main() {
    set_panic_hook();
    create_session_if_not_set_then(|_| handle_error(run_with_args(CLI::parse())));
}
