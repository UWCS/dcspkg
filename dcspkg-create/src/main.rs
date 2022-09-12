use clap::Parser;
use dcspkg_common::Package;
use std::path::{Path, PathBuf};

mod archive;
mod db;
mod opts;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let directory = PathBuf::from(&args.directory);
    println!("Creating new dcspkg from {directory:?}");
    println!("Please specify package options (skip to use defaults)");

    let pkg_name = opts::get_pkg_name(directory.file_name().and_then(|s| s.to_str()))?;
    let version = opts::get_version()?;

    db::validate_name_and_version(&args.db, &pkg_name, &version)?;

    let description = opts::get_description()?;
    let image_url = opts::get_image_url()?;
    let executable_path = opts::get_exe_path(&directory)?;
    let add_to_path = opts::add_to_path()?;
    let has_installer = opts::has_installer(&directory)?;
    let archive_name = format!("{pkg_name}-{version}.dcspkg");

    print!("Creating tarball...");
    let archive_path = args.pkg_dir.join(&archive_name);

    let crc = archive::make_archive(&archive_path, &directory)?;

    println!("done!");

    let mut package = Package {
        id: 0,
        name: pkg_name,
        description,
        version,
        image_url,
        archive_path: archive_name,
        executable_path,
        crc,
        has_installer,
        add_to_path,
    };

    package.id = db::add_package_to_db(&args.db, package.clone())?;

    println!("{}", serde_json::to_string_pretty(&package)?);

    println!("Added package to database");
    println!("Your package is now ready for download!");
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The directory to package up
    #[clap(validator = dir_exists)]
    directory: PathBuf,
    #[clap(short, long, value_parser, validator=file_exists)]
    #[clap(default_value = "packages/packagedb.sqlite")]
    db: PathBuf,
    #[clap(short, long, value_parser, validator=dir_exists)]
    #[clap(default_value = "packages/packages")]
    pkg_dir: PathBuf,
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
