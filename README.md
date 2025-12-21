Feeds — COSMIC applet (English)
=================================

Short description
-----------------

This is a small COSMIC panel applet that fetches and displays RSS/Atom
feed items. It uses `libcosmic` + `iced` for UI and `reqwest` + `rss`
for network parsing.

Prerequisites
-------------

To build the applet, you need the following dependencies:

### 1. Install Rust, Cargo, and Just

#### Arch Linux
```bash
sudo pacman -S rust just 
```

#### Fedora
```bash
sudo dnf install rust cargo just
```

#### Pop!_OS / Ubuntu
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install just
```

### 2. System Libraries
- **Pop!_OS / Ubuntu / Debian**: `sudo apt install libdbus-3.0-dev libgtk-3-dev`
- **Fedora**: `sudo dnf install dbus-devel gtk3-devel`
- **Arch Linux**: `sudo pacman -S dbus gtk3`

- **reqwest** and **rss**: These are included in the project's dependencies, so they'll be installed when you run `just`.

### Installation Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/marcossl10/cosmic-simple-feeds.git
   cd cosmic-simple-feeds
   git submodule update --init --recursive
   ``` 
   ...
2. sudo cargo build --release
   ...

3. Install the applet:
   ```bash
   sudo just install
   ```

4. If the applet icon is cached, log out and back in to refresh the session.

Configuration
-------------

The application uses `cosmic-config` to persist user feeds. By default
it includes a sample feed; manage feeds from the popup "Gerenciar"
view inside the applet.

Developers should install [rustup][rustup] and consider using
`rust-analyzer` in their editor.

[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
