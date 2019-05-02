//! A basic ZipReader/Writer crate

#![warn(missing_docs)]

#[cfg(feature = "bzip2")]
extern crate bzip2;
extern crate flate2;
extern crate msdos_time;
extern crate podio;
extern crate time;

pub use compression::CompressionMethod;
pub use read::ZipArchive;
pub use write::ZipWriter;

mod compression;
mod cp437;
mod crc32;
pub mod read;
pub mod result;
mod spec;
mod types;
pub mod write;
