use satlight_common::location::Position;

#[derive(Debug)]
pub struct Cursor<'a> {
    // Bytes is faster than Chars
    ptr: *const u8,
    offset: usize,
    #[cfg(debug_assertions)]
    prev: u8,
    pub length: usize,
    pub input: &'a str,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        profiling::scope!("Cursor::new");
        Self {
            ptr: input.as_ptr(),
            offset: 0,
            #[cfg(debug_assertions)]
            prev: 0,
            length: input.len(),
            input,
        }
    }

    pub fn position(&self) -> Position {
        let mut col = 1;
        let mut line = 1;
        for c in self.input[..self.offset()].chars() {
            if c == '\n' {
                col = 1;
                line += 1;
            } else {
                col += 1;
            }
        }
        Position::new(line, col, self.offset())
    }

    pub fn is_eof(&self) -> bool {
        self.offset >= self.length
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn prev(&self) -> u8 {
        #[cfg(debug_assertions)]
        {
            self.prev
        }
        #[cfg(not(debug_assertions))]
        {
            0
        }
    }

    pub fn current(&self) -> u8 {
        profiling::scope!("Cursor::current");
        if self.is_eof() {
            0
        } else {
            unsafe { *self.ptr.add(self.offset) }
        }
    }

    pub fn peek(&self) -> u8 {
        profiling::scope!("Cursor::peek");
        let offset = self.offset + 1;
        if offset >= self.length {
            0
        } else {
            unsafe { *self.ptr.add(offset) }
        }
    }

    pub fn mov(&mut self, offset: isize) -> u8 {
        #[cfg(debug_assertions)]
        {
            self.prev = self.current();
        }
        // safe abstraction :)
        let ch = unsafe { *self.ptr.offset(offset) };
        if offset < 0 {
            // as explicit
            self.offset -= offset.abs() as usize;
        } else {
            self.offset += offset as usize;
        }
        ch
    }

    pub fn shift(&mut self, offset: isize) -> u8 {
        #[cfg(debug_assertions)]
        {
            self.prev = self.current();
        }
        // safe abstraction :)
        let ch = unsafe { *self.ptr.offset(offset) };
        if offset < 0 {
            // as explicit
            self.offset -= offset.abs() as usize;
        } else {
            self.offset += offset as usize + 1;
        }
        ch
    }

    pub fn bump(&mut self) -> u8 {
        profiling::scope!("Cursor::bump");
        #[cfg(debug_assertions)]
        {
            self.prev = self.current();
        }
        let ch = unsafe { *self.ptr.add(self.offset) };
        self.offset += 1;
        ch
    }

    pub fn eat_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while predicate(self.current()) && !self.is_eof() {
            self.bump();
        }
    }
}
