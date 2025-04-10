use crate::source_map::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub src_id: SourceId,
    pub byte_offset: usize,
    pub byte_len: usize,
}
