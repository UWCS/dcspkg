use anyhow::Context;
use std::path::Path;

use dcspkg::Package;
use tabular::{Row, Table};

///helper to print a list of packages as a nice table
pub fn print_package_list(list: &[Package], raw: bool) {
    if raw {
        println!("{}", serde_json::to_string(list).unwrap());
    } else {
        if list.is_empty() {
            println!("Package list is empty!");
        }
        let mut table = Table::new("{:<}  {:<}  {:<}").with_row(
            Row::new()
                .with_cell("Game/App Name")
                .with_cell("Package Shortname")
                .with_cell("Description"),
        );
        for pkg in list {
            table.add_row(
                Row::new()
                    .with_cell(&pkg.fullname)
                    .with_cell(&pkg.pkgname)
                    .with_cell(pkg.description.as_deref().unwrap_or("-")),
            );
        }

        println!("{table}");
    }
}

/// Helper to get the list of packages from the json file on disk
pub fn get_registry(path: &Path) -> anyhow::Result<Vec<Package>> {
    std::fs::File::open(path)
        .context("Could not open registry file")
        .and_then(|reader| {
            serde_json::from_reader(reader).context("Could not parse JSON from registry")
        })
}
