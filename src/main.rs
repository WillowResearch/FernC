#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![allow(unused)]

//! The compiler for the Fern programming language.
//!
//! It consists of the following stages:
//! 1) The lexer, implemented in the module `lex`.
//! 2) TODO

use std::io::stdout;

use diagnostics::Diagnostic;
use lex::{lex_source, token::TokenTree};
use parse::{parse_source, visit::pretty_print};
use source_map::SourceMap;

mod diagnostics;
mod lex;
mod parse;
mod source_map;

type FResult<T> = Result<T, Vec<Diagnostic>>;

fn main() {
    let mut sm = SourceMap::new();
    sm.add_source_from_file("examples/simple.fern");

    match pipeline(&sm) {
        Ok(_) => {},
        Err(errs) => {
            let mut out = String::new();

            for e in errs {
                e.render(&mut out, &sm);
                out.push('\n');
            }

            print!("{out}");
        },
    }
}

fn pipeline(sm: &SourceMap) -> FResult<()> {
    let mut errors = Vec::new();

    for source in sm.sources() {
        match parse_source(&source) {
            Ok(parsed) => {
                let mut out = String::new();
                pretty_print(&parsed, source, &mut out);
                println!("{out}");
            },
            Err(e) => errors.extend(e),
        }
        
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}
