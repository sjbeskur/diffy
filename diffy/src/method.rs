use anyhow::{ Result};
use comde::{Compressor, Decompressor};
use std::{
    io::{self, Read, Seek, Write},
    str::FromStr,
};


/// Compression method used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Stored,
    Deflate,
    Brotli,
    Snappy,
    Zstd,
}

impl Default for Method {
    fn default() -> Self {
        Self::Stored
    }
}

impl Method {
    pub fn compress<W: Write + Seek, R: Read>(
        self,
        writer: &mut W,
        reader: &mut R,
    ) -> io::Result<comde::ByteCount> {
        match self {
            Self::Stored => comde::stored::StoredCompressor::new().compress(writer, reader),
            Self::Deflate => comde::deflate::DeflateCompressor::new().compress(writer, reader),
            Self::Brotli => comde::brotli::BrotliCompressor::new().compress(writer, reader),
            Self::Snappy => comde::snappy::SnappyCompressor::new().compress(writer, reader),
            Self::Zstd => comde::zstd::ZstdCompressor::new().compress(writer, reader),
        }
    }

    pub fn decompress<W: Write, R: Read>(self, reader: R, writer: W) -> io::Result<u64> {
        match self {
            Self::Stored => comde::stored::StoredDecompressor::new().copy(reader, writer),
            Self::Deflate => comde::deflate::DeflateDecompressor::new().copy(reader, writer),
            Self::Brotli => comde::brotli::BrotliDecompressor::new().copy(reader, writer),
            Self::Snappy => comde::snappy::SnappyDecompressor::new().copy(reader, writer),
            Self::Zstd => comde::zstd::ZstdDecompressor::new().copy(reader, writer),
        }
    }
}

impl FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stored" => Ok(Method::Stored),
            "deflate" => Ok(Method::Deflate),
            "brotli" => Ok(Method::Brotli),
            "snappy" => Ok(Method::Snappy),
            "zstd" => Ok(Method::Zstd),
            _ => Err(format!("Unknown compression method {}", s)),
        }
    }
}