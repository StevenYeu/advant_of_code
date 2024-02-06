pub mod days;
pub mod util;
use std::fs::File;
use std::io::{Read, Result};
fn main() -> Result<()> {
    let file_path = util::parse_args();
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let _ = days::day_one(&contents);
    return Ok(());
}
