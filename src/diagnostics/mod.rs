use crate::source_map::{Source, SourceMap, SourcePos, Span};
use render::DiagWriter;
use std::{
    fmt::{self, Write},
    io::repeat,
    usize,
};

mod render;
pub mod specifics;

struct DiagnosticPart {
    span: Span,
    help: String,
}

pub struct Diagnostic {
    msg: String,
    parts: Vec<DiagnosticPart>,
}

impl Diagnostic {
    pub fn new(msg: String) -> Self {
        Self {
            msg,
            parts: Vec::new(),
        }
    }

    pub fn add_part(mut self, span: Span, help: String) -> Self {
        self.parts.push(DiagnosticPart { span, help });
        self
    }

    pub fn render(&self, wr: &mut impl Write, sm: &SourceMap) -> Result<(), fmt::Error> {
        let mut writer = DiagWriter::new_ansi(wr);
        render::render(&mut writer, self, sm)
    }
}
