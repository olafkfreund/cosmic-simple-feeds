Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It uses `libcosmic` + `iced` for UI and `reqwest` + `rss`
for network parsing.

Prerequisites
-------------

Make sure you have Rust and Just installed on your system:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Just
cargo install just
```

Compile and Install
-------------------

Clone the repository and use just to install:

```bash
git clone https://github.com/marcossl10/cosmic-simple-feeds.git
cd cosmic-simple-feeds
git submodule update --init --recursive
just
sudo just install
```

If the applet icon appears cached in your COSMIC panel after install, log out and log back in to refresh the session.

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
