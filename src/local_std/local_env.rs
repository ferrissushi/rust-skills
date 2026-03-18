use std::{env::{args, current_dir, current_exe, home_dir, set_current_dir}, fs};
pub fn env_modules_main() -> std::io::Result<()> {
    let mut args = args();
    let second_args = args.nth(2).unwrap_or("no args".to_string());
    println!("{second_args}");


    let current_dir = current_dir()?;
    println!("{}", current_dir.display());

    let current_exe = current_exe()?;
    println!("{}", current_exe.display());

    let home_dir = home_dir().unwrap();
    println!("{}", home_dir.display());

    set_current_dir(home_dir)?;

    let current_dir_content = fs::read_dir("./")?;

    for file in current_dir_content {
        let content = file?;
        println!("{}", content.path().display());
    }

    Ok(())
}
