use memmap2::Mmap;

/// A low-level view into the underlying memory-mapped JImage.
#[derive(Debug)]
pub struct RawJImage<'a> {
    mmap: &'a Mmap,
}

impl<'a> RawJImage<'a> {
    /// Creates a new `RawJImage` instance from a memory-mapped file.
    pub(crate) fn new(mmap: &'a Mmap) -> Self {
        Self { mmap }
    }

    /// Returns total length of the mapped file in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    /// Returns a pointer to the start of the memory-mapped data.
    ///
    /// # Safety
    /// The caller must ensure that the pointer is not used after the `JImage` instance.
    /// Dereferencing this pointer is unsafe; it is valid for `self.len()` bytes.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.mmap.as_ptr()
    }

    /// Returns the entire memory-mapped data as a byte slice.
    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        &self.mmap
    }
}
