//!Utility to update files in directory
#![warn(missing_docs)]

use walkdir::WalkDir;
use std::path::Path;
use std::io;
use std::fs;
use std::fmt;
use std::error::Error;

///Describes potential errors
pub enum UpdateError {
    ///Directory from which to copy file is invalid
    InvalidFrom,
    ///Directory into which to copy file is invalid
    InvalidTo,
    ///Other IO errors.
    Io(io::Error),
}

impl Error for UpdateError {
    fn description(&self) -> &str {
        match self {
            UpdateError::InvalidFrom => "Directory from which to copy files doesn't exist or cannot be accessed",
            UpdateError::InvalidTo => "Directory into which to copy files doesn't exist or cannot be accessed",
            UpdateError::Io(_) => "I/O Error occurred",
        }
    }
}

impl fmt::Debug for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateError::Io(ref error) => write!(f, "I/O Error occurred: {}", error),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl fmt::Display for UpdateError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn skip_hidden(entry: &walkdir::DirEntry) -> bool {
    !entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

///Describes how to perform update
pub trait DirUpdate {
    #[inline]
    ///Hook to handle change to file
    fn on_new_file_update(_file: &Path) {
    }

    #[inline]
    ///Hook to handle no change to file
    fn on_new_file_skip(_file: &Path) {
    }

    ///Specifies general error handler
    fn on_io_error(file: &Path, error: io::Error) {
        eprintln!("{}: Error accessing: {}", file.display(), error);
    }

    ///Specifies error handler for `WalkDir`
    ///
    ///By default prints error to stderr.
    fn on_walk_error(error: walkdir::Error) {
        match (error.path(), error.io_error()) {
            (Some(path), Some(error)) => eprintln!("{}: Cannot access file. Error: {}", path.display(), error),
            (Some(path), None) => eprintln!("{}: Cannot access file", path.display()),
            (None, Some(error)) => eprintln!("I/O Error: {}", error),
            (None, None) => eprintln!("Unknown error happened"),
        }
    }

    #[doc(hidden)]
    fn filter_error(value: walkdir::Result<walkdir::DirEntry>) -> Option<walkdir::DirEntry> {
        match value {
            Ok(entry) => match entry.file_type().is_dir() {
                true => None,
                false => Some(entry),
            },
            Err(error) => {
                Self::on_walk_error(error);
                None
            }
        }
    }

    ///Performs update of `to` directory and returns number of updated files
    fn update_dir(from: &Path, to: &Path) -> Result<usize, UpdateError> {
        if !from.metadata().map(|from| from.is_dir()).unwrap_or(false) {
            return Err(UpdateError::InvalidFrom);
        } else if !to.metadata().map(|to| to.is_dir()).unwrap_or(false) {
            return Err(UpdateError::InvalidTo);
        }

        let mut result = 0;

        for to_entry in WalkDir::new(to).into_iter().filter_entry(skip_hidden).filter_map(Self::filter_error) {
            let to_path = to_entry.path();
            let to_short_path = to_path.strip_prefix(to).expect("To strip prefix of file in to directory");
            let from_path = from.join(to_short_path);

            let mut from_file = match fs::File::open(&from_path) {
                Ok(file) => file,
                Err(_) => continue
            };

            let from_size = match from_file.metadata() {
                Ok(meta) => meta.len(),
                Err(error) => {
                    Self::on_io_error(&from_path, error);
                    continue;
                }
            };

            let to_size = match to_path.metadata() {
                Ok(meta) => meta.len(),
                Err(error) => {
                    Self::on_io_error(&to_path, error);
                    continue;
                }
            };

            if to_size == from_size {
                Self::on_new_file_skip(&to_path);
                continue
            }

            let mut to_file = match fs::File::create(&to_path) {
                Ok(file) => file,
                Err(error) => {
                    Self::on_io_error(&to_path, error);
                    continue;
                }
            };

            match io::copy(&mut from_file, &mut to_file) {
                Ok(_) => {
                    Self::on_new_file_update(&to_path);
                    result += 1;
                },
                Err(error) => Self::on_io_error(to_path, error),
            }
        }

        Ok(result)
    }
}
