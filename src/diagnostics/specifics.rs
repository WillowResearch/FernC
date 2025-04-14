pub mod lex {
    use super::super::Diagnostic;
    use crate::source_map::{Source, Span};

    pub fn illegal_char(span: Span, source: &Source) -> Diagnostic {
        let sym_text = source.text_of_span(span);

        Diagnostic::new(format!("Illegal character `{sym_text}`.")) //
            .add_part(span, String::new())
    }
}
