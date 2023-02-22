use anyhow::{Result};
use diffy_patch::*;


fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::builder().init();

    let args = argh::from_env();
    
    do_patch(&args)?;

    Ok(())
}
