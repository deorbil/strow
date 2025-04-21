use std::path::{Path, PathBuf, StripPrefixError};

pub trait PathExt {
    fn replace_prefix(&self, from: &Path, to: &Path) -> Result<PathBuf, StripPrefixError>;
}

impl PathExt for PathBuf {
    fn replace_prefix(&self, from: &Path, to: &Path) -> Result<PathBuf, StripPrefixError> {
        Ok(to.join(self.strip_prefix(from)?))
    }
}
