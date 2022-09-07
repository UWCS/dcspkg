use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::{Compression, CrcWriter};
use std::fs::File;
use std::path::Path;

//returns crc
pub fn make_archive(dir_path: &Path, archive_name: &str) -> Result<u32> {
    let archive = File::create(archive_name)?;
    let encoder = GzEncoder::new(archive, Compression::default());
    let encoder = CrcWriter::new(encoder);
    let mut tar = tar::Builder::new(encoder);
    tar.append_dir_all(".", dir_path)?;
    Ok(tar.into_inner()?.crc().sum())
}
