Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It uses `libcosmic` + `iced` for UI and `reqwest` + `rss`
for network parsing.

Prerequisites
-------------

To install Cosmic Desktop, you need the following dependencies:

### System Dependencies
- **Rust**: Install via [rustup](https://rustup.rs/)
- **Just**: Install via Cargo: `cargo install just`
- **System Libraries**:
  - For Debian/Ubuntu: `sudo apt install libdbus-3.0-dev libgtk-3-dev`
  - For Fedora: `sudo dnf install dbus-devel gtk3-devel`
  - For Arch: `sudo pacman -S dbus gtk3`
- **libcosmic**: Install via Cargo: `cargo install libcosmic`
- **iced**: Install via Cargo: `cargo install iced`
- **reqwest** and **rss**: These are included in the project's dependencies, so they'll be installed when you run `just`.

### Installation Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/marcossl10/cosmic-simple-feeds.git
   cd cosmic-simple-feeds
   git submodule update --init --recursive
   ```

2. Install the applet:
   ```bash
   sudo just install
   ```

3. If the applet icon is cached, log out and back in to refresh the session.

Configuration
-------------

The application uses `cosmic-config` to persist user feeds. By default
it includes a sample feed; manage feeds from the popup "Gerenciar"
view inside the applet.

Developers should install [rustup][rustup] and consider using
`rust-analyzer` in their editor.

[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
