[![Build Status][github-actions-image]][github-actions-url]
[![MIT license][license-image]][license-url]

[github-actions-image]: https://img.shields.io/github/actions/workflow/status/aesy/m3u-to-spotify/ci.yml?branch=master&style=flat-square

[github-actions-url]: https://github.com/aesy/m3u-to-spotify/actions

[license-image]: https://img.shields.io/github/license/aesy/m3u-to-spotify?style=flat-square

[license-url]: https://github.com/aesy/m3u-to-spotify/blob/master/LICENSE

A Windows/Linux CLI tool for migrating/converting m3u/m3u8 playlists to Spotify.

## Usage

For download and usage instructions, check out the [readme for the CLI](./m3u-to-spotify-cli/README.md).

## Development

#### Prerequisites

* [Rust Stable 1.80.0+](https://www.rust-lang.org/tools/install)
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Build

To compile the project, simply issue the following command:

```sh
$ cargo build
```

#### Test

##### Linting

This project uses [rustfmt](https://github.com/rust-lang/rustfmt) for formatting and
[clippy](https://github.com/rust-lang/rust-clippy) for linting. Run them with:

```sh
$ cargo fmt 
$ cargo clippy
```

## Contribute

Use the [issue tracker](https://github.com/aesy/m3u-to-spotify/issues) to report bugs or make feature requests. Pull
requests
are welcome, but it may be a good idea to create an issue to discuss any changes beforehand.

## License

MIT, see [LICENSE](/LICENSE) file.
