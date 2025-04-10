use lex::lex_source;
use source_map::{Source, SourceMap};

mod lex;
mod source_map;
mod span;
mod token;

fn main() {
    let mut sm = SourceMap::new();
    let id = sm.add_source(Source {
        filename: "test.fern",
        text: r#"fn test() { print(hello); }"#,
    });

    dbg!(lex_source(&sm, id));
}
