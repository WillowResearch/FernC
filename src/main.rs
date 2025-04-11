#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(unused)]

//! The compiler for the Fern programming language.
//!
//! It consists of the following stages:
//! 1) The lexer, implemented in the module `lex`.
//! 2) TODO

use diagnostics::Diagnostic;
use lex::lex_source;
use source_map::SourceMap;
use token::TokenType;

mod diagnostics;
mod lex;
mod source_map;
mod token;

fn main() {
    let mut sm = SourceMap::new();
    sm.add_source_from_file("examples/simple.fern");

    let mut errors = String::new();

    for source in sm.sources() {
        for token in lex_source(source) {
            if token.ty() != TokenType::Error {
                continue;
            }

            let text = source.text_of_span(token.span());
            _ = Diagnostic::new(format!("Unexpected symbol `{text}`."))
                .add_part(token.span(), "Unexpected symbol".to_string())
                .render(&mut errors, &sm);
            errors.push_str("\n");
        }
    }

    print!("{errors}");
}
