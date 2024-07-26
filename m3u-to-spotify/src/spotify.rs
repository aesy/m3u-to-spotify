use anyhow::Error;
use rspotify::clients::{BaseClient, OAuthClient};
use rspotify::model::{
    FullPlaylist, FullTrack, PlayableId, PlaylistId, PlaylistResult, SearchResult, SearchType,
    UserId,
};
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};
use uuid::Uuid;

use crate::TrackTags;

#[must_use]
pub fn create_client(client_id: &str, client_secret: &str, redirect_url: &str) -> AuthCodeSpotify {
    let creds = Credentials::new(client_id, client_secret);
    let oauth = OAuth {
        redirect_uri: String::from(redirect_url),
        scopes: scopes!("playlist-modify-private", "user-library-modify"),
        state: Uuid::new_v4().to_string(),
        proxies: None,
    };

    AuthCodeSpotify::new(creds, oauth)
}

pub fn create_playlist(
    spotify: &impl OAuthClient,
    user_id: UserId<'_>,
    name: &str,
) -> anyhow::Result<FullPlaylist> {
    let playlist = spotify.user_playlist_create(user_id, name, Some(false), None, None)?;
    Ok(playlist)
}

pub fn add_tracks_to_playlist(
    spotify: &impl OAuthClient,
    playlist_id: PlaylistId<'_>,
    track_ids: &[FullTrack],
) -> anyhow::Result<PlaylistResult> {
    let tracks = track_ids
        .iter()
        .filter_map(|track| track.id.clone())
        .map(PlayableId::Track)
        .collect::<Vec<PlayableId>>();
    let result = spotify.playlist_add_items(playlist_id, tracks, None)?;

    Ok(result)
}

pub fn search_track(
    spotify: &impl BaseClient,
    tags: &TrackTags,
) -> anyhow::Result<Option<FullTrack>> {
    let mut parts = Vec::new();

    if let Some(term) = &tags.term {
        parts.push(term.to_string());
    }

    if let Some(title) = &tags.title {
        parts.push(format!("track:{title}"));
    }

    if let Some(artist) = &tags.artist {
        parts.push(format!("artist:{artist}"));
    }

    if let Some(album) = &tags.album {
        parts.push(format!("album:{album}"));
    }

    if let Some(isrc) = &tags.isrc {
        parts.push(format!("isrc:{isrc}"));
    }

    if parts.is_empty() {
        return Ok(None);
    }

    let query = parts.join(" ");
    let result = spotify.search(&query, SearchType::Track, None, None, Some(1), None)?;

    match result {
        SearchResult::Tracks(page) => Ok(page.items.first().cloned()),
        _ => Err(Error::msg("Wrong result type")),
    }
}
