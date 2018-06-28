use std::fs::File;
use std::io::Read;
use std::path::Path;

use flate2::read::GzDecoder;

use util::u32_from_slice;

use super::units;

/// Dictionary class for retrieval and binary I/O.
#[derive(Debug, Clone)]
pub struct Dictionary {
    /// Root index
    pub root: u32,
    pub units: Vec<u32>,
}

impl Dictionary {
    /// Reads a dictionary from a file.
    pub fn from_file(p: &Path) -> Self {
        Self::from_stream(&mut GzDecoder::new(File::open(p).unwrap()))
    }

    /// Reads a dictionary from an input stream.
    pub fn from_stream<T>(reader: &mut T) -> Self
    where
        T: Read,
    {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf).unwrap();

        let base_size = u32::from_le(u32_from_slice(&buf[..])) as usize;
        let buf_size = base_size * 4;

        let mut buf: Vec<u8> = vec![0; buf_size];
        assert_eq!(buf_size, buf.capacity());
        reader.read_exact(&mut buf[0..buf_size]).unwrap();
        assert_eq!(buf_size, buf.len());
        assert_eq!(buf_size, buf.capacity());

        let mut units: Vec<u32> = Vec::with_capacity(base_size);
        units.extend(buf.chunks(4).map(|ch| u32::from_le(u32_from_slice(ch))));
        assert_eq!(base_size, units.len());
        assert_eq!(base_size, units.capacity());

        let root = 0;

        Dictionary { root, units }
    }

    /// Checks if a given index is related to the end of a key.
    pub fn has_value(&self, index: u32) -> bool {
        units::has_leaf(self.units[index as usize], None)
    }

    /// Gets a value from a given index.
    pub fn value(&self, index: u32) -> u32 {
        let offset = units::offset(self.units[index as usize]);
        let value_index = (index ^ offset) & units::PRECISION_MASK;
        units::value(self.units[value_index as usize], None)
    }

    /// Exact matching.
    pub fn contains(&self, key: &str) -> bool {
        let index = self.follow_bytes(key, self.root);
        match index {
            Some(index) => self.has_value(index),
            None => false,
        }
    }

    /// Exact matching (returns value)
    pub fn find(&self, key: &str) -> Option<u32> {
        let index = self.follow_bytes(key, self.root);
        match index {
            Some(index) => match self.has_value(index) {
                true => Some(self.value(index)),
                false => None,
            },
            None => None,
        }
    }

    /// Follows a transition
    pub fn follow_char(&self, label: u8, index: u32) -> Option<u32> {
        trace!(r#"dawg::Dictionary::follow_char() "#);
        trace!(r#" label: {}, index = {} "#, label, index);
        let ttt = self.units[index as usize];
        trace!(r#" ttt: {} "#, ttt);
        let offset = units::offset(ttt);
        trace!(r#" offset: {} "#, offset);
        let next_index = (index ^ offset ^ u32::from(label)) & units::PRECISION_MASK;
        trace!(
            r#" units::label(): {} "#,
            units::label(self.units[next_index as usize], None)
        );
        if units::label(self.units[next_index as usize], None) == u32::from(label) {
            return Some(next_index);
        }
        None
    }

    /// Follows transitions.
    pub fn follow_bytes(&self, s: &str, index: u32) -> Option<u32> {
        let mut index = index;
        for ch in s.as_bytes() {
            index = match self.follow_char(*ch, index) {
                Some(v) => v,
                None => return None,
            };
        }
        Some(index)
    }
}
