use dcspkg::Package;
use tabular::{Row, Table};

pub fn print_package_list(list: &[Package], raw: bool) -> Option<()> {
    if raw {
        println!("{}", serde_json::to_string(list).unwrap());
    } else {
        let mut table = Table::new("{:<}   {:<}").with_row(
            Row::new()
                .with_cell("Package Name")
                .with_cell("Description"),
        );
        for pkg in list {
            table.add_row(
                Row::new()
                    .with_cell(&pkg.fullname)
                    .with_cell(pkg.description.as_deref().unwrap_or("-")),
            );
        }

        println!("{table}");
    }
    Some(())
}
