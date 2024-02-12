#[derive(Debug, Clone)]
pub struct Span {
    pub stream: String,
    pub row: usize,
    pub column: usize,
    pub index: usize,
    pub length: usize
}

impl Span {
    pub fn new(stream: String, row: usize, column: usize, index: usize, length: usize) -> Self {
        Self {
            stream,
            row,
            column,
            index,
            length
        }
    }
}
