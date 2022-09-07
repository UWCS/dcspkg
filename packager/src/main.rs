use clap::Parser;
use dcspkg_server::Package;
use std::path::{Path, PathBuf};

mod archive;
mod db;
mod opts;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    dbg!(&args);
    let directory = PathBuf::from(&args.directory);
    println!("Creating new dcspkg from {directory:?}");
    println!("Please specify package options (skip to use defaults)");

    let pkg_name = opts::get_pkg_name(directory.file_name().and_then(|s| s.to_str()))?;
    let version = opts::get_version()?;

    db::validate_name_and_version(&args.db, &pkg_name, &version)?;

    let description = opts::get_description()?;
    let url = opts::get_image_url()?;
    let exe_path = opts::get_exe_path()?;
    let add_to_path = opts::add_to_path()?;
    let has_installer = opts::has_installer(&directory)?;
    let archive_name = format!("{pkg_name}-{version}.dcspkg");

    archive::make_archive(&directory, &archive_name)?;

    let package = Package::default();

    db::add_package_to_db(&args.db, package)?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The directory to package up
    #[clap(validator = dir_exists)]
    directory: String,
    #[clap(short, long, value_parser, validator=file_exists)]
    #[clap(default_value = "packagedb.sqlite")]
    db: String,
    #[clap(short, long, value_parser, validator=dir_exists)]
    #[clap(default_value = "packages")]
    pkg_dir: String,
}

fn dir_exists(f: &str) -> Result<(), &'static str> {
    Path::new(f)
        .is_dir()
        .then_some(())
        .ok_or("Directory does not exist")
}

fn file_exists(f: &str) -> Result<(), &'static str> {
    Path::new(f)
        .is_file()
        .then_some(())
        .ok_or("File does not exist")
}
