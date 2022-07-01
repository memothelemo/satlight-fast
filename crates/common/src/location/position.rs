use satlight_macros::Getter;

#[derive(Debug, Clone, Copy, Getter, PartialEq, Eq, Hash)]
pub struct Position {
    #[clone]
    pub(crate) line: usize,
    #[clone]
    pub(crate) column: usize,
    #[clone]
    pub(crate) offset: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }
}

impl Position {
    pub fn with_line(self, line: usize) -> Self {
        Self { line, ..self }
    }

    pub fn with_column(self, column: usize) -> Self {
        Self { column, ..self }
    }

    pub fn with_offset(self, offset: usize) -> Self {
        Self { offset, ..self }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}
