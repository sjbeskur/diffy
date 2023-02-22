use anyhow::{Context, Result};
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


pub fn do_diff(
    Diff {
        older,
        newer,
        patch,
        method,
        sort_partitions,
        scan_chunk_size,
    }: &Diff,
) -> Result<()> {
    println!("Using method {:?}", method);
    let start = Instant::now();

    let older_contents = fs::read(older).context("read old file")?;
    let newer_contents = fs::read(newer).context("read new file")?;

    let (mut patch_r, mut patch_w) = pipe::pipe();
    let diff_params = DiffParams::new(*sort_partitions, *scan_chunk_size).unwrap();
    std::thread::spawn(move || {
        bidiff::simple_diff_with_params(
            &older_contents[..],
            &newer_contents[..],
            &mut patch_w,
            &diff_params,
        )
        .context("simple diff with params")
        .unwrap();
    });

    let mut compatch_w = BufWriter::new(File::create(patch).context("create patch file")?);
    method
        .compress(&mut compatch_w, &mut patch_r)
        .context("write output file")?;
    compatch_w.flush().context("finish writing output file")?;

    info!("Completed in {:?}", start.elapsed());

    Ok(())
}