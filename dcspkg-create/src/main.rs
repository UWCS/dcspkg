use clap::Parser;
use dcspkg::Package;
use std::io::Write;
use std::path::PathBuf;
mod archive;
mod db;
mod opts;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let directory = PathBuf::from(&args.directory);
    println!("Creating new dcspkg from {directory:?}");
    println!("Please specify package options (skip to use defaults)");

    let pkgname = opts::get_pkg_name(directory.file_name().and_then(|s| s.to_str()))?;

    db::check_name_unique(&args.db, &pkgname)?;

    let fullname = opts::get_full_name(&pkgname)?;
    let description = opts::get_description()?;
    let image_url = opts::get_image_url()?;
    let executable_path = opts::get_exe_path(&directory)?;
    let add_to_path = opts::add_to_path()?;
    let has_installer = opts::has_installer(&directory)?;

    print!("Creating tarball...");
    std::io::stdout().flush()?; //print with no newline so force a flush

    let archive_path = args.pkg_dir.join(format!("{pkgname}.dcspkg"));

    let crc = archive::make_archive(&archive_path, &directory)?;

    println!("done!");

    let package = Package {
        pkgname,
        description,
        image_url,
        executable_path,
        crc,
        has_installer,
        add_to_path,
        fullname,
    };

    println!("{}", serde_json::to_string_pretty(&package)?);

    db::add_package_to_db(&args.db, package)?;

    println!("Added package to database");
    println!("Your package is now ready for download!");
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The directory to package up
    #[arg(value_parser = dir_exists)]
    directory: PathBuf,
    #[arg(short, long, value_parser, value_parser=file_exists)]
    #[arg(default_value = "packages/packagedb.sqlite")]
    db: PathBuf,
    #[arg(short, long, value_parser, value_parser=dir_exists)]
    #[arg(default_value = "packages/packages")]
    pkg_dir: PathBuf,
}

fn dir_exists(f: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(f);
    if !path.is_dir() {
        Err("Directory does not exist")
    } else {
        Ok(path)
    }
}

fn file_exists(f: &str) -> Result<PathBuf, &'static str> {
    let path = PathBuf::from(f);
    if !path.is_file() {
        Err("File does not exist")
    } else {
        Ok(path)
    }
}
