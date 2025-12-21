Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It uses `libcosmic` + `iced` for UI and `reqwest` + `rss`
for network parsing.

Build & install
---------------

Build with Cargo or use the provided `justfile` for convenience:

```bash
cargo build --release
sudo just install
```

Configuration
-------------

The application uses `cosmic-config` to persist user feeds. By default
it includes a sample feed; manage feeds from the popup "Gerenciar"
view inside the applet.

Contributing
------------

Feel free to open issues or pull requests. The project is licensed
under the MIT License (see `LICENSE`).

Developers should install [rustup][rustup] and consider using
`rust-analyzer` in their editor.

[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
