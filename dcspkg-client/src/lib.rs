mod commands;

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/download";
const LIST_ENDPOINT: &str = "/list";

pub use commands::install::install;
pub use commands::list::list;
