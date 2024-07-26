use std::path::PathBuf;

use clap::Parser;
use clap::ValueHint;

/// Tool for migrating/converting m3u/m3u8 playlists to Spotify.
///
/// This program reads a music playlist of type m3u or m3u3 (extended utf8-encoded m3u) to
/// search for each song on Spotify and adding it to a playlist. The search is enhanced by
/// using the id3 tags of the music file, if possible.
///
/// The result (whether a song was found/added or not) and errors are printed to stderr.
#[derive(Parser, Debug)]
#[command(name = "m3u-to-spotify", version, verbatim_doc_comment)]
pub struct Args {
    /// The client ID of the spotify app to use to authenticate the user.
    #[arg(long = "client-id", env = "CLIENT_ID", verbatim_doc_comment)]
    pub client_id: String,

    /// The client secret of the spotify app to use to authenticate the user.
    #[arg(long = "client-secret", env = "CLIENT_SECRET", verbatim_doc_comment)]
    pub client_secret: String,

    /// The redirect URL of the spotify app to use to authenticate the user.
    #[arg(long = "redirect-url", env = "REDIRECT_URL", verbatim_doc_comment)]
    pub redirect_url: String,

    /// The path to the m3u/m3u8 file to migrate/convert.
    #[arg(
        long = "playlist-path",
        env = "PLAYLIST_PATH",
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        verbatim_doc_comment,
    )]
    pub playlist_path: PathBuf,

    /// The name of the playlist to create and add all songs to.
    ///
    /// A new playlist is created even if a playlist of the same name already exists.
    #[arg(
        long = "playlist-name",
        env = "PLAYLIST_NAME",
        default_value = "m3u-to-spotify",
        verbatim_doc_comment
    )]
    pub playlist_name: Option<String>,
}
