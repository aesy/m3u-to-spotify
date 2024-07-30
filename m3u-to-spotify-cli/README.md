# m3u-to-spotify CLI

## Usage

CLI options and examples can be viewed via the CLI's help command:

```shell
m3u-to-spotify --help
```

```
Tool for exporting m3u/m3u8 playlists to Spotify.

This program reads a music playlist of type m3u or m3u8 (extended utf8-encoded m3u) to
search for each song on Spotify and adding it to a Spotify playlist. The search is enhanced
by using the id3 tags of the music file, if possible.

The result (whether a song was found/added or not) is printed to stdout and errors to stderr.

Usage: m3u-to-spotify [OPTIONS] --client-id <CLIENT_ID> --client-secret <CLIENT_SECRET> --redirect-url <REDIRECT_URL> --playlist-path <FILE>

Options:
      --client-id <CLIENT_ID>
          The client ID of the spotify app to use to authenticate the user.

          [env: CLIENT_ID=]

      --client-secret <CLIENT_SECRET>
          The client secret of the spotify app to use to authenticate the user.

          [env: CLIENT_SECRET=]

      --redirect-url <REDIRECT_URL>
          The redirect URL of the spotify app to use to authenticate the user.

          [env: REDIRECT_URL=]

      --playlist-path <FILE>
          The path to the m3u/m3u8 file to export.

          [env: PLAYLIST_PATH=]

      --playlist-name <PLAYLIST_NAME>
          The name of the Spotify playlist to create and add all songs to.

          A new playlist is created even if a playlist of the same name already exists.

          [env: PLAYLIST_NAME=]
          [default: m3u-to-spotify]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Spotify App

As you may have noticed, in order to use the CLI you need to provide a client ID, client secret and a redirect URL.
This information can be obtained by logging into the
[Spotify Developer console](https://developer.spotify.com/dashboard/login) and creating an app. Add any name and
description you like and a local URL to the list of redirect URIs, for example http://localhost:3000. Also make sure to
check "Web API" under "APIs used".

Once saved, the Client ID and Client secret can be found under "Basic information". Use the same redirect URL as the one
you provided previously.

## Installation:

### Binaries

The latest stable `m3u-to-spotify` releases are available
as [prebuilt binaries for 64-bit Linux and Windows under Github
releases](https://github.com/aesy/m3u-to-spotify/releases). Binaries for other platforms have to be built from source.

#### Linux

Example on how to download and install an `m3u-to-spotify` binary using [curl](https://curl.se/):

```shell
curl -L https://github.com/aesy/m3u-to-spotify/releases/download/v0.1.0/m3u-to-spotify-linux-amd64 -o m3u-to-spotify
chmod +x m3u-to-spotify
sudo mv m3u-to-spotify /usr/local/bin
```

#### Windows

Example on how to download an `m3u-to-spotify` binary using [curl](https://curl.se/):

```shell
curl -L https://github.com/aesy/m3u-to-spotify/releases/download/v0.1.0/m3u-to-spotify-windows-amd64.exe -o m3u-to-spotify.exe
```

#### Build from source

To build the CLI, run the following command:

```shell
cargo build --bin m3u-to-spotify-cli --target <target> --release
```

The resulting binary can be found in the repositories root directory under `target/<target>/release/` called
`m3u-to-spotify-cli`.

The `<target>` options can be found
at [Rust's platform support page](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
