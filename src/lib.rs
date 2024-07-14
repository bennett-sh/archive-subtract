use decompress::{decompress, ExtractOptsBuilder};
use pathdiff::diff_paths;
use std::{env::temp_dir, fs, io, path::PathBuf};
use thiserror::Error;
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Error, Debug)]
pub enum Error {
  #[error("failed to build extract options")]
  ExtractOptsBuilderError,
  #[error("decompress error: {0}")]
  DecompressError(decompress::DecompressError),
  #[error("io error: {0}")]
  IOError(io::Error),
}

pub fn subtract(
  subtraction_file: impl Into<PathBuf>,
  target: impl Into<PathBuf>,
) -> Result<(), Error> {
  let temp = temp_dir().join(Uuid::new_v4().to_string());
  let target_path = target.into();

  decompress(
    subtraction_file.into(),
    temp.clone(),
    &ExtractOptsBuilder::default()
      .build()
      .map_err(|_| Error::ExtractOptsBuilderError)?,
  )
  .map_err(Error::DecompressError)?;

  for file in WalkDir::new(&temp)
    .into_iter()
    .filter_map(Result::ok)
    .filter(|e| !e.file_type().is_dir())
  {
    let path_in_target: Option<PathBuf> =
      diff_paths(file.path(), &temp).map(|path| target_path.join(path));

    if let Some(path) = path_in_target {
      if path.exists() {
        fs::remove_file(path).map_err(Error::IOError)?;
      }
    }
  }

  fs::remove_dir_all(temp).map_err(Error::IOError)?;

  Ok(())
}
