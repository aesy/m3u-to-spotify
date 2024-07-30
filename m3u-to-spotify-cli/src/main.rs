use std::process::ExitCode;

use clap::Parser;
use indicatif::ProgressBar;
use rspotify::prelude::OAuthClient;

use m3u_to_spotify::{playlist, spotify, track};

use crate::args::Args;

mod args;

fn main() -> ExitCode {
    let Args {
        client_id,
        client_secret,
        redirect_url,
        playlist_path,
        playlist_name,
    } = Args::parse();

    let spotify = spotify::create_client(&client_id, &client_secret, &redirect_url);

    let url = match spotify.get_authorize_url(false) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Failed to get authorization URL: '{e}'");
            return ExitCode::FAILURE;
        },
    };

    while let Err(e) = spotify.prompt_for_token(&url) {
        println!("Failed to get token from URL. Please try again. '{e}'");
    }

    let user = match spotify.me() {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Failed to fetch authorized Spotify user: '{e}'");
            return ExitCode::FAILURE;
        },
    };

    let playlist_name = playlist_name
        .or_else(|| {
            playlist_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| String::from("m3u-to-spotify"));

    let playlist = match spotify::create_playlist(&spotify, user.id, &playlist_name) {
        Ok(playlist) => playlist,
        Err(e) => {
            eprintln!("Failed to get or create Spotify playlist: '{e}'");
            return ExitCode::FAILURE;
        },
    };

    let tracks = match playlist::read(&playlist_path) {
        Ok(tracks) => tracks,
        Err(e) => {
            eprintln!("Failed to read M3U playlist: '{e}'");
            return ExitCode::FAILURE;
        },
    };

    let progress = ProgressBar::new(tracks.len() as u64);
    let mut buffer = Vec::with_capacity(100);

    for track in tracks {
        let path = track.to_string_lossy();
        let info = track::read_id3_tags(&track);

        match spotify::search_track(&spotify, &info) {
            Ok(Some(track)) => {
                println!("Found '{}' for track '{path}'", track.name);
                buffer.push(track);
            },
            Ok(None) => {
                println!("No results found for track '{path}''");
            },
            Err(e) => {
                eprintln!("Failed to search to track '{path}': '{e}''");
            },
        }

        if buffer.len() >= buffer.capacity() {
            if let Err(e) = spotify::add_tracks_to_playlist(&spotify, playlist.id.clone(), &buffer)
            {
                eprintln!("Failed to add tracks to playlist: '{e}''");
            }
            buffer.clear();
        }

        progress.inc(1);
    }

    if !buffer.is_empty() {
        if let Err(e) = spotify::add_tracks_to_playlist(&spotify, playlist.id.clone(), &buffer) {
            eprintln!("Failed to add tracks to playlist: '{e}''");
        }
        buffer.clear();
    }

    println!("Done!\nAll tracks found has been added to playlist '{playlist_name}'!");

    ExitCode::SUCCESS
}
