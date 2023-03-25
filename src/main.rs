#[path="func/sub.rs"]
mod sub;
use sub::{sub, full_name};

fn main() {
    sub("imran");
    full_name();
}
