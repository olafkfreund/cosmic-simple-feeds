Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It was adapted from the COSMIC app template and uses
`libcosmic` + `iced` for UI and `reqwest` + `rss` for network parsing.

Build & install
---------------

Build with Cargo or use the provided `justfile` for convenience:

```bash
cargo build --release
sudo just install
```

Run
---

After installing, add the applet to your COSMIC panel (application
entry: `com.github-pop-os.cosmic-app-template`). If the panel shows a
cached icon, try logging out and logging back in to refresh the session.

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
