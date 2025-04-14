use super::Diagnostic;
use crate::{
    diagnostics::DiagnosticPart,
    source_map::{Source, SourceMap, SourcePos, Span},
};
use std::fmt::{self, Write};

pub fn render<W: Write>(
    wr: &mut DiagWriter<W>,
    diag: &Diagnostic,
    sm: &SourceMap,
) -> Result<(), fmt::Error> {
    use DiagnosticRenderLine as DRL;

    let mut lines = Vec::new();

    // Arrange the parts by their start position. TODO: handle overlapping
    // parts.
    let mut parts: Vec<&DiagnosticPart> = diag.parts.iter().collect();
    parts.sort_by_key(|p| p.span.start().byte());

    // First we assemble all the lines to be rendered.
    for part in parts {
        let source = sm.get_source(part.span.src_id());

        let start_line = source.line_of(part.span.start());
        let end_line = source.line_of(part.span.end());
        assert!(start_line == end_line, "TODO: handle multi line messages");

        lines.push(DRL::SourcePos(part.span.start()));
        lines.push(DRL::Padding);

        for line in start_line..=end_line {
            lines.push(DRL::CodeLine { source, line });

            // If this is the line the highlight is on then we add it here.
            if line == start_line {
                lines.push(DRL::Highlight {
                    span: part.span,
                    message: &part.help,
                });
            }
        }

        lines.push(DRL::Padding);
    }

    // Combine certain line types:
    let all_lines = lines;
    let mut lines = Vec::new();

    for i in 0..all_lines.len() {
        if all_lines[i].is_padding() && i > 0 && all_lines[i - 1].can_collapse_padding() {
            continue;
        }

        lines.push(all_lines[i]);
    }

    // Now we can perform the actual rendering.
    wr.write_error(&diag.msg)?;

    let gutter_width = lines.iter().map(DRL::gutter_width).max().unwrap_or(0);

    for line in lines {
        match line {
            DRL::SourcePos(pos) => {
                let source = sm.get_source(pos.src_id());
                wr.write_source_pos(pos, source, gutter_width)?;
            }
            DRL::Padding => wr.write_padding(gutter_width)?,
            DRL::CodeLine { source, line } => wr.write_code(source, line, gutter_width)?,
            DRL::Highlight { span, message } => {
                let source = sm.get_source(span.src_id());
                wr.write_highlight(source, span, gutter_width, &message)?;
            }
        }
    }

    Ok(())
}

const BOLD: &str = "\x1b[1m";
const RED_FG: &str = "\x1b[91m";
const BLUE_FG: &str = "\x1b[94m";
const RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
enum DiagnosticRenderLine<'a> {
    SourcePos(SourcePos),
    Padding,
    CodeLine { source: &'a Source, line: usize },
    Highlight { span: Span, message: &'a String },
}

impl<'a> DiagnosticRenderLine<'a> {
    fn is_padding(&self) -> bool {
        use DiagnosticRenderLine as DRL;
        matches!(self, DRL::Padding)
    }

    fn can_collapse_padding(&self) -> bool {
        use DiagnosticRenderLine as DRL;
        matches!(self, DRL::Highlight { .. })
    }

    fn gutter_width(&self) -> usize {
        use DiagnosticRenderLine as DRL;

        match self {
            DRL::SourcePos(_) => 0,
            DRL::Padding => 0,
            DRL::CodeLine { line, .. } => (line.ilog10() + 1) as usize,
            DRL::Highlight { .. } => 0,
        }
    }
}

pub struct DiagWriter<'a, W: Write> {
    wr: &'a mut W,
}

impl<'a, W: Write> DiagWriter<'a, W> {
    pub fn new_ansi(wr: &'a mut W) -> Self {
        Self { wr }
    }

    fn write_error(&mut self, msg: &str) -> Result<(), fmt::Error> {
        writeln!(self.wr, "{RED_FG}{BOLD}error{RESET}{BOLD}: {msg}{RESET}")
    }

    fn write_source_pos(
        &mut self,
        pos: SourcePos,
        source: &Source,
        gw: usize,
    ) -> Result<(), fmt::Error> {
        writeln!(
            self.wr,
            "{}{BLUE_FG}{BOLD}-->{RESET} {}:{}:{}",
            " ".repeat(gw),
            source.filename(),
            source.line_of(pos),
            source.col_of(pos)
        )
    }

    fn write_padding(&mut self, gw: usize) -> Result<(), fmt::Error> {
        writeln!(self.wr, "{}{BLUE_FG}{BOLD} |{RESET}", " ".repeat(gw))
    }

    fn write_code(&mut self, source: &Source, line: usize, gw: usize) -> Result<(), fmt::Error> {
        let line_span = source.span_of_line(line);
        let text = source.text_of_span(line_span);
        writeln!(
            self.wr,
            "{BLUE_FG}{BOLD}{0:1$} |{RESET} {2}",
            line, gw, text
        )
    }

    fn write_highlight(
        &mut self,
        source: &Source,
        span: Span,
        gw: usize,
        msg: &str,
    ) -> Result<(), fmt::Error> {
        assert!(source.line_of(span.start()) == source.line_of(span.end()));

        let offset = source.col_of(span.start()) - 1;

        // TODO: Actual text length.
        let len = source.text_of_span(span).chars().count();
        let highlight_text = "^".repeat(len);

        writeln!(
            self.wr,
            "{}{BLUE_FG}{BOLD} | {RESET}{}{RED_FG}{BOLD}{} {}{RESET}",
            " ".repeat(gw),
            " ".repeat(offset),
            highlight_text,
            msg
        )
    }
}
