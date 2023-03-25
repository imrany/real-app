#[path="func/sub.rs"]
mod sub;
use sub::{sub, full_name};
// mod gcsf;

// pub use gcsf::filesystem::{Gcsf, NullFs};
fn main() {
    sub("imran");
    full_name();
}
