pub mod playlist;
pub mod spotify;
pub mod track;

#[derive(Clone, Debug, Default)]
pub struct TrackTags {
    term: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    isrc: Option<String>,
}
