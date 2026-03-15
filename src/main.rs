use std::{fs, os::unix::fs::PermissionsExt};

mod local_std;

fn main() -> std::io::Result<()> {
    let data = fs::metadata("./src/local_std/local_fs.rs")?;
    let permissions = data.permissions();
    let permissions_bits: u32 = permissions.mode();
    println!("{permissions_bits}");
    Ok(())
}
