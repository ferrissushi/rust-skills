use crate::local_std::local_cmp::launch_cmp_module_test;

mod local_std;

fn main() -> std::io::Result<()> {
    launch_cmp_module_test()?;

    Ok(())

}
