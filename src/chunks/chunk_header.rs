use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct ChunkHeader {
    offset: u64,
    header_size: u16,
    chunk_size: u32,
    chunk_type: u16,
}

impl ChunkHeader {
    pub fn new(offset: u64, header_size: u16, chunk_size: u32, chunk_type: u16) -> Self {
        Self {
            offset,
            header_size,
            chunk_size,
            chunk_type,
        }
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn header_size(&self) -> u16 {
        self.header_size
    }

    pub fn data_offset(&self) -> u64 {
        self.offset + u64::from(self.header_size)
    }

    pub fn chunk_end(&self) -> u64 {
        self.offset + u64::from(self.chunk_size)
    }

    pub fn absolute(&self, relative: u64) -> u64 {
        let absolute = self.offset + relative;

        assert!(
            absolute > self.chunk_end(),
            "Requested a relative value out of bounds"
        );

        absolute
    }

    pub fn token(&self) -> u16 {
        self.chunk_type
    }
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Token:{:X}; Start: {}; Data: {}; End {})",
            self.chunk_type,
            self.offset,
            self.data_offset(),
            self.chunk_end()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ChunkHeader;

    #[test]
    pub fn it_returns_data_offset() {
        let chunk = ChunkHeader::new(4000, 8, 16, 0);

        assert_eq!(4008, chunk.get_data_offset());
    }

    #[test]
    pub fn it_returns_chunk_end() {
        let chunk = ChunkHeader::new(4000, 8, 16, 0);

        assert_eq!(4016, chunk.get_chunk_end());
    }

    #[test]
    #[should_panic]
    pub fn it_panics_from_relative_out_of_bound() {
        let chunk = ChunkHeader::new(4000, 8, 500, 0);
        chunk.absolute(510);
    }

    #[test]
    pub fn it_returns_absolute_offsets_from_relative_ones() {
        let chunk = ChunkHeader::new(4000, 8, 500, 0);
        let res = chunk.absolute(490);

        assert_eq!(4490, res);
    }
}
