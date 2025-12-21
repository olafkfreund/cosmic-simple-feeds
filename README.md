Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It was adapted from the COSMIC app template and uses
`libcosmic` + `iced` for UI and `reqwest` + `rss` for network parsing.

Build & install
---------------

You can build the project with Cargo (or use the provided `justfile`):

```bash
cargo build --release
sudo just install
```

Run
---

After installing, add the applet to your COSMIC panel (the application
entry is `com.github-pop-os.cosmic-app-template`). If the panel shows a
cached icon, try logging out and logging back in to refresh the session.

Configuration
-------------

The application uses `cosmic-config` to persist user feeds. By default
it contains one sample feed; manage feeds from the popup "Gerenciar"
view inside the applet.

Contributing
------------

Feel free to open issues or pull requests. The project follows the
MPL-2.0 license (see `LICENSE`).

License
-------

This project is distributed under the MIT License. See `LICENSE`.
# Feeds

An application for the COSMIC™ desktop

## Installation

A [justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/feeds prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer].

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[sccache]: https://github.com/mozilla/sccache
