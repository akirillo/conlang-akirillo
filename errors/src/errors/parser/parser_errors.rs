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

use std::fmt::{Debug, Display};

create_messages!(
    /// ParserError enum that represents all the errors for the  `conlang-parser` crate.
    ParserError,
    code_mask: 0000i32,
    code_prefix: "PAR",

    /// For when the parser encountered an unexpected token.
    @formatted
    unexpected_token {
        args: (message: impl Display),
        msg: message,
        help: None,
    }

        /// For when the parser encountered an unexpected End of File.
    @formatted
    unexpected_eof {
        args: (),
        msg: "unexpected EOF",
        help: None,
    }

    /// For when the parser encountered an unexpected list of tokens.
    @formatted
    unexpected {
        args: (found: impl Display, expected: impl Display),
        msg: format!("expected {expected} -- found '{found}'"),
        help: None,
    }

    /// When more input was expected but not found.
    @backtraced
    lexer_empty_input {
        args: (),
        msg: "Expected more characters to lex but found none.",
        help: None,
    }

    /// When a block comment is empty.
    @backtraced
    lexer_empty_block_comment {
        args: (),
        msg: "Empty block comment.",
        help: None,
    }

    /// When a block comment is not closed before end of file.
    @backtraced
    lexer_block_comment_does_not_close_before_eof {
        args: (input: impl Display),
        msg: format!("Block comment does not close with content: `{input}`."),
        help: None,
    }

    /// When the lexer could not lex some text.
    @backtraced
        could_not_lex {
        args: (input: impl Display),
        msg: format!("Could not lex the following content: `{input}`.\n"),
        help: None,
    }

    /// For when the lexer encountered a bidi override character
    @backtraced
    lexer_bidi_override {
        args: (),
        msg: "Unicode bidi override code point encountered.",
        help: None,
    }

    /// When a hex number is provided.
    @backtraced
    lexer_hex_number_provided {
        args: (input: impl Display),
        msg: format!("A hex number `{input}..` was provided but hex is not allowed."),
        help: None,
    }

    /// When a block comment cannot be lexed.
    @backtraced
    could_not_lex_block_comment {
        args: (),
        msg: format!("Block comment cannot be lexed`."),
        help: None,
    }

    /// When a constant cannot be parsed into a field element.
    @formatted
    constant_cannot_be_parsed_into_field_element {
        args: (constant: impl Display),
        msg: format!("Constant `{constant}` cannot be parsed into a field element."),
        help: None,
    }



);
