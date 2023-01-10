use std::{
    borrow::Cow,
    ffi::OsStr,
    fs, io,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use codespan::{ByteIndex, ColumnIndex, LineIndex, LineOffset, Location, RawIndex, Span};
use fs2::FileExt;
use memmap2::Mmap;
use owning_ref::{CloneStableAddress, StableAddress};

#[derive(Debug, Clone)]
/// Can be cheaply cloned.
pub struct File(Arc<FileInner>);

impl File {
    pub fn open(path: PathBuf) -> io::Result<Self> {
        Ok(Self(Arc::new(FileInner::new(path)?)))
    }

    #[inline]
    fn inner(&self) -> &FileInner {
        &self.0
    }

    #[inline]
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.inner().metadata()
    }

    #[inline]
    pub fn path(&self) -> &Path {
        self.inner().path()
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.inner().data()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner().source_str()
    }
}

impl AsRef<[u8]> for File {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<str> for File {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for File {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

unsafe impl StableAddress for File {}
unsafe impl CloneStableAddress for File {}

#[derive(Debug)]
struct FileInner {
    path: PathBuf,
    inner: fs::File,
    mmap: Mmap,
    line_starts: Vec<ByteIndex>,
}

impl FileInner {
    fn new(path: PathBuf) -> io::Result<Self> {
        // Open it
        let file = fs::File::open(&path)?;
        // Lock the file
        file.lock_shared()?;
        // Get the memory map.
        let mmap = unsafe { Mmap::map(&file)? };

        let s = match std::str::from_utf8(&mmap) {
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "File contains invalid UTF-8",
                ));
            }
            Ok(s) => s,
        };

        let inner = FileInner {
            path,
            inner: file,
            line_starts: codespan_reporting::files::line_starts(s)
                .map(|u| ByteIndex(u as u32))
                .collect(),
            mmap,
        };

        Ok(inner)
    }

    fn last_line_index(&self) -> LineIndex {
        LineIndex::from(self.line_starts.len() as RawIndex)
    }

    fn line_index(&self, byte_index: ByteIndex) -> LineIndex {
        match self.line_starts.binary_search(&byte_index) {
            // Found the start of a line
            Ok(line) => LineIndex::from(line as u32),
            Err(next_line) => LineIndex::from(next_line as u32 - 1),
        }
    }

    fn line_span(&self, line_index: LineIndex) -> Result<Span, codespan_reporting::files::Error> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + LineOffset::from(1))?;

        Ok(Span::new(line_start, next_line_start))
    }

    fn line_start(
        &self,
        line_index: LineIndex,
    ) -> Result<ByteIndex, codespan_reporting::files::Error> {
        use std::cmp::Ordering;

        match line_index.cmp(&self.last_line_index()) {
            Ordering::Less => Ok(self.line_starts[line_index.to_usize()]),
            Ordering::Equal => Ok(self.source_span().end()),
            Ordering::Greater => Err(codespan_reporting::files::Error::LineTooLarge {
                given: line_index.to_usize(),
                max: self.last_line_index().to_usize(),
            }),
        }
    }

    fn source_span(&self) -> Span {
        Span::from_str(self.source_str())
    }

    fn source_slice(&self, span: Span) -> Result<&str, codespan_reporting::files::Error> {
        let start = span.start().to_usize();
        let end = span.end().to_usize();

        self.source_str().get(start..end).ok_or_else(|| {
            let max = self.source_str().len() - 1;
            codespan_reporting::files::Error::IndexTooLarge {
                given: if start > max { start } else { end },
                max,
            }
        })
    }

    fn location(
        &self,
        byte_index: ByteIndex,
    ) -> Result<Location, codespan_reporting::files::Error> {
        let line_index = self.line_index(byte_index);
        let line_start_index = self.line_start(line_index).map_err(|_| {
            codespan_reporting::files::Error::IndexTooLarge {
                given: byte_index.to_usize(),
                max: self.source_str().len() - 1,
            }
        })?;
        let line_src = self
            .source_str()
            .get(line_start_index.to_usize()..byte_index.to_usize())
            .ok_or_else(|| {
                let given = byte_index.to_usize();
                let max = self.source_str().len() - 1;
                if given > max {
                    codespan_reporting::files::Error::IndexTooLarge { given, max }
                } else {
                    codespan_reporting::files::Error::InvalidCharBoundary { given }
                }
            })?;

        Ok(Location {
            line: line_index,
            column: ColumnIndex::from(line_src.chars().count() as u32),
        })
    }

    #[inline]
    fn data(&self) -> &[u8] {
        &self.mmap
    }

    #[inline]
    fn source_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.mmap) }
    }

    #[inline]
    fn metadata(&self) -> io::Result<fs::Metadata> {
        self.path.metadata()
    }

    #[inline]
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for FileInner {
    #[inline]
    fn drop(&mut self) {
        self.inner.unlock().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId(usize);

impl FileId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }
}

impl From<usize> for FileId {
    fn from(i: usize) -> Self {
        Self(i)
    }
}

impl From<FileId> for usize {
    fn from(f: FileId) -> Self {
        f.0
    }
}

#[derive(Debug, Clone)]
pub struct Files {
    files: Vec<File>,
}

impl Files {
    pub fn new() -> Self {
        Self { files: vec![] }
    }
    pub(crate) fn add(&mut self, file: File) -> io::Result<FileId> {
        let id = FileId(self.files.len());
        self.files.push(file);
        Ok(id)
    }

    pub fn line_span(
        &self,
        id: FileId,
        line_index: impl Into<LineIndex>,
    ) -> Result<Span, codespan_reporting::files::Error> {
        self.inner(id)?.line_span(line_index.into())
    }

    pub fn line_index(
        &self,
        id: FileId,
        byte_index: impl Into<ByteIndex>,
    ) -> Result<LineIndex, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.line_index(byte_index.into()))
    }

    pub fn location(
        &self,
        id: FileId,
        byte_index: impl Into<ByteIndex>,
    ) -> Result<Location, codespan_reporting::files::Error> {
        self.inner(id)?.location(byte_index.into())
    }

    pub fn source(&self, id: FileId) -> Result<&str, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.source_str())
    }

    pub fn source_data(&self, id: FileId) -> Result<&[u8], codespan_reporting::files::Error> {
        Ok(self.inner(id)?.data())
    }

    pub fn file_name(&self, id: FileId) -> Result<&OsStr, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.path.file_name().unwrap())
    }

    pub fn source_slice(
        &self,
        id: FileId,
        span: Span,
    ) -> Result<&str, codespan_reporting::files::Error> {
        self.inner(id)?.source_slice(span)
    }

    pub fn source_span(&self, id: FileId) -> Result<Span, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.source_span())
    }

    pub fn get(&self, id: FileId) -> Result<&File, codespan_reporting::files::Error> {
        self.files
            .get(id.0)
            .ok_or(codespan_reporting::files::Error::FileMissing)
    }

    fn inner(&self, id: FileId) -> Result<&FileInner, codespan_reporting::files::Error> {
        self.files
            .get(id.0)
            .map(|f| f.inner())
            .ok_or(codespan_reporting::files::Error::FileMissing)
    }
}

impl<'a> codespan_reporting::files::Files<'a> for Files {
    type FileId = FileId;

    type Name = Cow<'a, str>;

    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.path().to_string_lossy())
    }

    fn source(
        &'a self,
        id: Self::FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        Ok(self.inner(id)?.source_str())
    }

    fn line_index(
        &'a self,
        id: Self::FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        Ok(self
            .inner(id)?
            .line_index(ByteIndex(byte_index as u32))
            .to_usize())
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        let span = self.line_span(id, line_index as u32)?;

        Ok(span.start().to_usize()..span.end().to_usize())
    }
}
