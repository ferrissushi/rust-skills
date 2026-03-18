use crate::local_std::{local_clone::clone_module_main, local_cmp::launch_cmp_module_test, local_collections::collection_module_main, local_default::default_module_main, local_env::env_modules_main, local_iterator::iterator_module_main, local_path::path_module_main};

mod local_std;

fn main() -> std::io::Result<()> {
    launch_cmp_module_test()?;
    default_module_main()?;
    path_module_main()?;
    iterator_module_main()?;
    collection_module_main()?;
    env_modules_main()?;
    clone_module_main()?;
    Ok(())

}
