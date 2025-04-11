use std::{fs::read_to_string, ops::Range};

/// A struct to manage and own all the `Source`s the compiler uses.
#[derive(Debug, Default)]
pub struct SourceMap {
    /// The list of `Source`s. The index in this vector is the `SourceId` of a
    /// `Source`.
    sources: Vec<Source>,
}

impl SourceMap {
    /// Create a new, empty `SourceMap`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new `Source` from the name `filename` and content `text`.
    ///
    /// Returns the `SourceId` of the newly created `Source`.
    pub fn add_source(&mut self, filename: String, text: String) -> SourceId {
        let id = SourceId(self.sources.len());
        let source = Source::new(id, filename, text);
        self.sources.push(source);

        id
    }

    /// Load a `Source` from the file with the given name.
    ///
    /// Panics if the file doesn't exist.
    pub fn add_source_from_file(&mut self, filename: &str) -> SourceId {
        let text = read_to_string(filename).unwrap();
        self.add_source(filename.to_owned(), text)
    }

    /// Returns the `Source` with the given id.
    pub fn get_source(&self, id: SourceId) -> &Source {
        &self.sources[id.0]
    }

    /// An iterator over the `Source`s in the map.
    pub fn sources(&self) -> impl Iterator<Item = &Source> {
        self.sources.iter()
    }
}

/// A literal or virtual file from which source code is read.
#[derive(Debug)]
pub struct Source {
    /// The id this source has in its `SourceMap`.
    id: SourceId,

    /// The name associated with this source. Note that this may not literally
    /// be a file name if the code does not come from disk.
    filename: String,

    /// The text content of the source file.
    text: String,

    /// The cached indices of all '\n' characters in the `text`. This is used
    /// to efficiently compute line numbers.
    newlines: Vec<usize>,
}

impl Source {
    fn compute_newlines(text: &str) -> Vec<usize> {
        let mut newlines: Vec<usize> = text.match_indices('\n').map(|(i, _)| i).collect();

        // We make sure the last line is terminated.
        if text.chars().next_back() != Some('\n') {
            newlines.push(text.len());
        }

        newlines
    }

    /// Create a new source file with the given id from our `SourceMap`,
    /// `filename`, and `text` content.
    fn new(id: SourceId, filename: String, text: String) -> Self {
        Self {
            id,
            filename,
            newlines: Self::compute_newlines(&text),
            text,
        }
    }

    /// Get the id this source has in its `SourceMap`.
    pub fn id(&self) -> SourceId {
        self.id
    }

    /// Get the name associated with this source. Note that this may not literally
    /// be a file name if the code does not come from disk.
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Get the text content of the source file.
    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn text_of_span(&self, span: Span) -> &str {
        &self.text[span.byte_range()]
    }

    /// Get the `SourcePos` for the given byte offset. `byte` should we aligned
    /// with the start of a utf8 boundary.
    ///
    /// This should be the only way to create a `SourcePos`.
    fn pos_from_byte(&self, byte: usize) -> SourcePos {
        assert!(self.text().is_char_boundary(byte));
        SourcePos::new(self.id(), byte)
    }

    /// The 1-indexed line number of the given position within this source.
    ///
    /// The newline for a line, if it exists, is considered part of the line
    /// it ends.
    pub fn line_of(&self, pos: SourcePos) -> usize {
        assert!(pos.src_id() == self.id());

        match self.newlines.binary_search(&pos.byte()) {
            // This is exactly a newline which is the last character on that line
            Ok(i) => i + 1,
            // This is between newlines in which case we want the index before
            // the newline which is luckily what `binary_search` gives.
            Err(i) => i + 1,
        }
    }

    /// The first byte of the 1-indexed line.
    fn first_byte_of_line(&self, line: usize) -> usize {
        let line = line - 1;
        if line == 0 {
            0 // The first line starts at byte 0.
        } else {
            // Other lines start at one past the end of the previous line.
            self.newlines[line - 1] + 1
        }
    }

    /// The 1-indexed column number of the given position within this source.
    pub fn col_of(&self, pos: SourcePos) -> usize {
        assert!(pos.src_id() == self.id());

        let line = self.line_of(pos);
        let start_byte = self.first_byte_of_line(line);

        pos.byte() - start_byte + 1
    }

    /// Get the span in this source that starts at the inclusive byte index
    /// `start` and ends at the exclusive byte index `end`.
    pub fn span(&self, start: usize, end: usize) -> Span {
        let start_pos = self.pos_from_byte(start);
        let end_pos = self.pos_from_byte(end);
        Span::new(start_pos, end_pos)
    }

    /// Get the span in this source that starts at the inclusive byte index
    /// start and has the given length.
    pub fn span_with_len(&self, start: usize, len: usize) -> Span {
        self.span(start, start + len)
    }

    /// Gives the span of the text on the given line, not including the final newline.
    pub(crate) fn span_of_line(&self, line: usize) -> Span {
        let start = self.first_byte_of_line(line);
        let end = self.first_byte_of_line(line + 1) - 1; // Remove the newline.
        self.span(start, end)
    }
}

/// An identifier for a `Source`. Use this as a handle to retrieve the `Source`
/// from the `SourceMap`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceId(usize);

/// A range of characters within a `Source`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The inclusive start position of the range.
    start: SourcePos,

    /// The exclusive end position of the range.
    end: SourcePos,
}

impl Span {
    /// Create a new `Span` from the inclusive start and exclusive end position.
    pub fn new(start: SourcePos, end: SourcePos) -> Self {
        assert!(start.src_id() == end.src_id());
        assert!(start.byte() <= end.byte());

        Self { start, end }
    }

    /// The id of the `Source` this `Span` is within.
    pub fn src_id(&self) -> SourceId {
        self.start.src_id()
    }

    /// Get the inclusive start position of the range.
    pub fn start(&self) -> SourcePos {
        self.start
    }

    /// Get the exclusive end position of the range.
    pub fn end(&self) -> SourcePos {
        self.end
    }

    /// The range of bytes within the `Source` the range includes.
    pub fn byte_range(&self) -> Range<usize> {
        self.start().byte()..self.end().byte()
    }
}

/// A position of a single character within a `Source`.
///
/// The byte offset here should always be aligned to a utf8 codepoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePos {
    /// The id of the `Source` that this position is within.
    src_id: SourceId,

    /// The byte offset into the `Source`.
    byte: usize,
}

impl SourcePos {
    pub fn new(src_id: SourceId, byte: usize) -> Self {
        Self { src_id, byte }
    }

    /// Get the id of the `Source` that this position is within.
    pub fn src_id(&self) -> SourceId {
        self.src_id
    }

    /// Get the byte offset into the `Source`.
    pub fn byte(&self) -> usize {
        self.byte
    }
}
