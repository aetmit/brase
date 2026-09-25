use std::{fs, io, path::{Path, PathBuf}};

pub struct SourceFile {
    pub path: PathBuf,
    pub source: Vec<u8>,
}

pub struct SourceManager;

impl SourceManager {
    pub fn read<T: AsRef<Path>>(&self, path: T) -> io::Result<SourceFile> {
        let path = path.as_ref().to_owned();
        let source = fs::read(&path)?;
        Ok(SourceFile { path, source })     
    }
}