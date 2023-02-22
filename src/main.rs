use anyhow::{Result};
use diffy::*;


fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_BACKTRACE", "1");

    //env_logger::builder().init();
    let Cli { cmd } = argh::from_env();
    match cmd {
        Command::Diff(args) => {
            do_diff(&args)?;
        }
        Command::Patch(args) => {
            do_patch(&args)?;
        }
        Command::Cycle(args) => {
            do_cycle(&args)?;
        }
    }
    
    Ok(())
}
