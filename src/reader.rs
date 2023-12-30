use std::io::{self, Read};

const BUF_SIZE: usize = 8 * 1024;

pub struct Reader<'a>(pub &'a mut dyn Read);

impl<'a> Read for Reader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

pub struct Iter<'a> {
    reader: Reader<'a>,
    buf: Box<[u8]>,
    len: usize,
    pos: usize,
}

impl<'a> Iter<'a> {
    pub fn new(reader: Reader<'a>) -> Self {
        Self {
            reader,

            buf: Box::new([0u8; BUF_SIZE]),
            len: 0,
            pos: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Need to buffer the next slice from file
        if self.len == 0 {
            self.len = self.reader.read(&mut self.buf[..]).ok()?;
            self.pos = 0;
        }

        // Check bounds for a possibly incomplete buffer. self.buf.get will
        // check bounds on the buffer, which may not completely filled with read bytes.
        if self.pos < self.len {
            self.pos += 1;
            self.buf.get(self.pos - 1).map(|c| *c as char)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for Reader<'a> {
    type Item = char;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
