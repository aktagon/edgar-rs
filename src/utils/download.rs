//! Utilities for downloading and extracting files.
//!
//! This module contains utility functions for downloading and extracting files.
//! These functions are only available when using the native feature.

#[cfg(feature = "native")]
use std::fs::File;
#[cfg(feature = "native")]
use std::io::{self, Write};
#[cfg(feature = "native")]
use std::path::{Path, PathBuf};

use crate::error::{EdgarApiError, Result};

/// Writes bytes to a temporary file.
///
/// # Parameters
///
/// * `bytes` - The bytes to write.
///
/// # Returns
///
/// The path to the temporary file.
#[cfg(feature = "native")]
pub fn write_temp_file(bytes: &[u8]) -> Result<PathBuf> {
    let temp_file = tempfile::NamedTempFile::new()
        .map_err(|e| EdgarApiError::request(format!("Failed to create temporary file: {}", e)))?;

    let path = temp_file.path().to_path_buf();

    // Write the bytes to the temporary file
    let mut file = File::create(&path)
        .map_err(|e| EdgarApiError::request(format!("Failed to create file: {}", e)))?;

    file.write_all(bytes)
        .map_err(|e| EdgarApiError::request(format!("Failed to write file: {}", e)))?;

    Ok(path)
}

/// Extracts a ZIP file.
///
/// # Parameters
///
/// * `zip_path` - The path to the ZIP file.
/// * `output_dir` - The directory to extract the ZIP file to.
///
/// # Returns
///
/// `Ok(())` if the extraction is successful, an error otherwise.
#[cfg(feature = "native")]
pub fn extract_zip(zip_path: &Path, output_dir: &Path) -> Result<()> {
    let file = File::open(zip_path)
        .map_err(|e| EdgarApiError::request(format!("Failed to open ZIP file: {}", e)))?;

    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| EdgarApiError::zip(format!("Failed to read ZIP archive: {}", e)))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| EdgarApiError::zip(format!("Failed to access ZIP file entry: {}", e)))?;

        let outpath = file
            .enclosed_name()
            .map(|p| output_dir.join(p))
            .ok_or_else(|| EdgarApiError::zip("Invalid file path in ZIP"))?;

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| {
                EdgarApiError::request(format!("Failed to create directory: {}", e))
            })?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).map_err(|e| {
                        EdgarApiError::request(format!("Failed to create directory: {}", e))
                    })?;
                }
            }

            let mut outfile = File::create(&outpath)
                .map_err(|e| EdgarApiError::request(format!("Failed to create file: {}", e)))?;

            io::copy(&mut file, &mut outfile)
                .map_err(|e| EdgarApiError::request(format!("Failed to write file: {}", e)))?;
        }
    }

    Ok(())
}


#[cfg(all(test, feature = "native"))]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn test_write_temp_file() {
        let data = b"Hello, world!";
        let path = write_temp_file(data).unwrap();

        // Read the file back
        let mut file = File::open(path).unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();

        assert_eq!(contents, data);
    }

}
