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

pub fn do_cycle(
    Cycle {
        older,
        newer,
        method,
        sort_partitions,
        scan_chunk_size,
    }: &Cycle,
) -> Result<()> {
    info!("Reading older and newer in memory...");
    let (older, newer) = (fs::read(older)?, fs::read(newer)?);

    info!(
        "Before {}, After {}",
        Size::from_bytes(older.len()),
        Size::from_bytes(newer.len()),
    );

    let mut compatch = Vec::new();
    let before_diff = Instant::now();

    {
        let mut compatch_w = io::Cursor::new(&mut compatch);

        let (mut patch_r, mut patch_w) = pipe::pipe();
        thread::scope(|s| {
            s.spawn(|_| {
                bidiff::simple_diff_with_params(
                    &older[..],
                    &newer[..],
                    &mut patch_w,
                    &DiffParams::new(*sort_partitions, *scan_chunk_size).unwrap(),
                )
                .context("simple diff with params")
                .unwrap();
                // this is important for `.compress()` to finish.
                // since we're using scoped threads, it's never dropped
                // otherwise.
                drop(patch_w);
            });
            method
                .compress(&mut compatch_w, &mut patch_r)
                .context("compress")
                .unwrap();
        })
        .unwrap();
    }

    let diff_duration = before_diff.elapsed();

    let ratio = (compatch.len() as f64) / (newer.len() as f64);

    let mut fresh = Vec::new();
    let before_patch = Instant::now();
    {
        let mut older = io::Cursor::new(&older[..]);

        let (patch_r, patch_w) = pipe::pipe();

        thread::scope(|s| {
            s.spawn(|_| {
                method
                    .decompress(&compatch[..], patch_w)
                    .context("decompress")
                    .unwrap();
            });

            let mut r = bipatch::Reader::new(patch_r, &mut older)
                .context("read patch")
                .unwrap();
            let fresh_size = io::copy(&mut r, &mut fresh).unwrap();

            assert_eq!(fresh_size as usize, newer.len());
        })
        .unwrap();
    }
    let patch_duration = before_patch.elapsed();

    let newer_hash = hmac_sha256::Hash::hash(&newer[..]);
    let fresh_hash = hmac_sha256::Hash::hash(&fresh[..]);

    anyhow::ensure!(newer_hash == fresh_hash, "Hash mismatch!");

    let cm = format!("{:?}", method);
    let cp = format!("patch {}", Size::from_bytes(compatch.len()));
    let cr = format!(
        "{:03.3}% of {}",
        ratio * 100.0,
        Size::from_bytes(newer.len())
    );
    let cdd = format!("dtime {:?}", diff_duration);
    let cpd = format!("ptime {:?}", patch_duration);
    println!("{:12} {:20} {:27} {:20} {:20}", cm, cp, cr, cdd, cpd);

    Ok(())
}

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