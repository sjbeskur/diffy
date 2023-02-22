use anyhow::{Context, Result};
use crossbeam_utils::thread;
use bidiff::DiffParams;
use log::*;
use size::Size;
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    time::Instant,
};

mod cli;
mod method;

pub use cli::*;
pub use method::*;


pub fn do_patch(
    Patch {
        older,
        patch,
        output,
        method,
    }: &Patch,
) -> Result<()> {
    println!("Using method {:?}", method);
    let start = Instant::now();

    let compatch_r = BufReader::new(File::open(patch).context("open patch file")?);
    let (patch_r, patch_w) = pipe::pipe();
    let method = *method;

    std::thread::spawn(move || {
        method
            .decompress(compatch_r, patch_w)
            .context("decompress")
            .unwrap();
    });

    let older_r = File::open(older)?;
    let mut fresh_r = bipatch::Reader::new(patch_r, older_r).context("read patch")?;
    let mut output_w = BufWriter::new(File::create(output).context("create patch file")?);
    io::copy(&mut fresh_r, &mut output_w).context("write output file")?;

    info!("Completed in {:?}", start.elapsed());

    Ok(())
}

