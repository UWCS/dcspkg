use dcspkg_client::Package;
use tabular::{Row, Table};

pub fn print_package_list(list: &[Package]) {
    let mut table = Table::new("{:<}   {:<}   {:<}").with_row(
        Row::new()
            .with_cell("Package Name")
            .with_cell("Version")
            .with_cell("Description"),
    );
    for pkg in list {
        table.add_row(
            Row::new()
                .with_cell(&pkg.name)
                .with_cell(&pkg.version)
                .with_cell(pkg.description.as_ref().unwrap_or(&Default::default())),
        );
    }

    println!("{table}");
}
