pub mod lex {
    use std::fmt::format;

    use super::super::Diagnostic;
    use crate::source_map::{Source, Span};

    pub fn illegal_char(span: Span, source: &Source) -> Diagnostic {
        let sym_text = source.text_of_span(span);

        Diagnostic::new(format!("Illegal character `{sym_text}`.")) //
            .add_part(span, String::new())
    }

    pub fn unmatched_open_paren(span: Span, source: &Source) -> Diagnostic {
        let paren_text = source.text_of_span(span);

        Diagnostic::new(format!("Unclosed delimiter `{paren_text}`."))
            .add_part(span, "has no match".to_owned())
    }

    pub fn unmatched_close_paren(span: Span, source: &Source) -> Diagnostic {
        let paren_text = source.text_of_span(span);

        Diagnostic::new(format!("Unexpected closing delimiter `{paren_text}`."))
            .add_part(span, "has no match".to_owned())
    }

    pub fn mismatched_close_paren(
        open_span: Span,
        close_span: Span,
        source: &Source,
    ) -> Diagnostic {
        let close_text = source.text_of_span(close_span);

        Diagnostic::new(format!("Mismatched closing delimiter `{close_text}`."))
            .add_part(open_span, "unclosed delimiter".to_owned())
            .add_part(close_span, "mismatched closing delimiter".to_owned())
    }
}
