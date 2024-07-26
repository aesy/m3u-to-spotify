use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use id3::{Frame, Tag, TagLike};

use crate::TrackTags;

#[must_use]
pub fn read_id3_tags(path: &Path) -> TrackTags {
    let Ok(file) = File::open(path) else {
        return TrackTags {
            term: Some(
                path.file_stem()
                    .map_or_else(|| path.to_string_lossy(), |name| name.to_string_lossy())
                    .to_string(),
            ),
            ..Default::default()
        };
    };

    let reader = BufReader::new(file);
    let Ok(tag) = Tag::read_from2(reader) else {
        return TrackTags {
            term: Some(
                path.file_stem()
                    .map_or_else(|| path.to_string_lossy(), |name| name.to_string_lossy())
                    .to_string(),
            ),
            ..Default::default()
        };
    };

    TrackTags {
        term: None,
        title: tag.title().map(str::to_string),
        artist: tag
            .album_artist()
            .or_else(|| tag.artist())
            .map(str::to_string),
        album: tag.album().map(str::to_string),
        isrc: tag.get("TSRC").map(Frame::to_string),
    }
}
