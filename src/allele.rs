use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Span {
    pub rid: u32,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RawSpan {
    pub rid: String,
    pub start: u32,
    pub end: u32,
}

impl RawSpan {
    pub fn new(rid: String, start: u32, end: u32) -> Self {
        RawSpan { rid, start, end }
    }
}

impl Span {
    pub fn new(rid: u32, start: u32, end: u32) -> Self {
        Span { rid, start, end }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.rid == other.rid && self.end > other.start && other.end > self.start
    }

    pub fn contigious(&self, other: &Self) -> bool {
        self.rid == other.rid && (self.end == other.start || other.end == self.start)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Allele {
    pos: Span,
    seq: Vec<u8>,
}

impl Allele {
    fn new(pos: Span, seq: Vec<u8>) -> Self {
        Allele { pos, seq }
    }
}
