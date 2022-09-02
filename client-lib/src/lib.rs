mod install;
mod list;

const DATA_ENDPOINT: &str = "/pkgdata";
const FILE_ENDPOINT: &str = "/pkg";
const PKGDIR: &str = "$HOME/.dcspkg";

pub use install::install;
