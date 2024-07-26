use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use m3u::Entry;

pub fn read(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let file = File::open(path)?;

    read_m3u8(&file).or_else(|_| read_m3u(&file))
}

fn read_m3u(file: &File) -> anyhow::Result<Vec<PathBuf>> {
    let reader = BufReader::new(file);
    let mut reader = m3u::Reader::new(reader);
    let playlist = reader.entries().collect::<anyhow::Result<Vec<_>, _>>()?;
    let mut tracks = Vec::with_capacity(playlist.len());

    for entry in playlist {
        let path = match entry {
            Entry::Path(path) => path,
            Entry::Url(url) => {
                println!("Unsupported path type URL. Ignoring '{url}'");
                continue;
            },
        };

        let path = if path.is_absolute() {
            path
        } else {
            fs::canonicalize(path.clone()).unwrap_or(path)
        };

        tracks.push(path);
    }

    Ok(tracks)
}

fn read_m3u8(file: &File) -> anyhow::Result<Vec<PathBuf>> {
    let reader = BufReader::new(file);
    let mut reader = m3u::Reader::new_ext(reader)?;
    let playlist = reader.entry_exts().collect::<anyhow::Result<Vec<_>, _>>()?;
    let mut tracks = Vec::with_capacity(playlist.len());

    for entry in playlist {
        let path = match entry.entry {
            Entry::Path(path) => path,
            Entry::Url(url) => {
                println!("Unsupported path type URL. Ignoring '{url}'");
                continue;
            },
        };

        let path = if path.is_absolute() {
            path
        } else {
            fs::canonicalize(path.clone()).unwrap_or(path)
        };

        tracks.push(path);
    }

    Ok(tracks)
}
