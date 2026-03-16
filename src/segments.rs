use std::path::Path;

use object::{Object, ObjectSegment};
use rangemap::RangeInclusiveMap;

pub struct ElfSegments {
    pub segments: RangeInclusiveMap<u64, Vec<u8>>,
}

impl ElfSegments {
    /// Create a new empty `ElfSegments`.
    pub fn new() -> Self {
        ElfSegments {
            segments: RangeInclusiveMap::new(),
        }
    }

    /// Create an `ElfSegments` by reading and parsing an ELF file, inserting
    /// all non-empty segments.
    pub fn from_elf(path: &Path) -> anyhow::Result<Self> {
        let mut segs = Self::new();
        segs.extend_from_elf(path)?;
        Ok(segs)
    }

    /// Insert a single segment covering `address..=address + (data.len() - 1)`.
    pub fn insert(&mut self, address: u64, data: Vec<u8>) {
        if data.is_empty() {
            return;
        }
        let end = address + u64::try_from(data.len() - 1).unwrap();
        self.segments.insert(address..=end, data);
    }

    /// Read an ELF file and insert all of its non-empty segments.
    pub fn extend_from_elf(&mut self, path: &Path) -> anyhow::Result<()> {
        let buffer = std::fs::read(path)?;
        let object = object::File::parse(&*buffer)?;
        self.extend_from_object(&object)?;
        Ok(())
    }

    /// Insert all non-empty segments from an already-parsed object file.
    pub fn extend_from_object<'data: 'file, 'file>(
        &mut self,
        object: &'file impl Object<'data, 'file>,
    ) -> anyhow::Result<()> {
        for seg in object.segments() {
            if seg.size() == 0 {
                continue;
            }
            self.insert(seg.address(), seg.data()?.to_vec());
        }
        Ok(())
    }
}
