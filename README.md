Feeds — COSMIC applet
=====================

A small COSMIC panel applet that fetches and displays RSS/Atom feed
items. Uses `libcosmic` + `iced` for UI and `reqwest` + `rss` for
network parsing.

Features
--------

- Multiple RSS/Atom feed support with parallel fetching
- In-applet feed management (add/remove feeds)
- Live configuration updates via `cosmic-config`
- Internationalization (English and Brazilian Portuguese)
- SSRF protection and URL validation
- Background refresh without UI flicker

Installation
------------

### NixOS (Flake)

Add the flake input to your `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cosmic-simple-feeds.url = "github:marcossl10/cosmic-simple-feeds";
  };
}
```

#### System-wide (NixOS module)

```nix
# configuration.nix or a NixOS module
{ inputs, ... }:
{
  imports = [ inputs.cosmic-simple-feeds.nixosModules.default ];

  # Add the overlay so the package is available
  nixpkgs.overlays = [ inputs.cosmic-simple-feeds.overlays.default ];

  # Enable the applet
  programs.cosmic-simple-feeds.enable = true;
}
```

#### Per-user (Home Manager module)

```nix
# home.nix or a Home Manager module
{ inputs, ... }:
{
  imports = [ inputs.cosmic-simple-feeds.homeManagerModules.default ];

  # Add the overlay
  nixpkgs.overlays = [ inputs.cosmic-simple-feeds.overlays.default ];

  programs.cosmic-simple-feeds = {
    enable = true;

    # Declarative feed list (optional — can also manage from the UI)
    feeds = [
      "https://planet.nixos.org/rss20.xml"
      "https://weekly.nixos.org/feeds/all.rss.xml"
      "https://news.google.com/rss?hl=en-US&gl=US&ceid=US:en"
    ];

    # Refresh interval in seconds (default: 300)
    refreshInterval = 600;
  };
}
```

#### Build only

```bash
nix build github:marcossl10/cosmic-simple-feeds
./result/bin/feeds
```

### Traditional (non-Nix)

#### Prerequisites

Install Rust, Cargo, and Just:

- **Arch Linux**: `sudo pacman -S rust just`
- **Fedora**: `sudo dnf install rust cargo just`
- **Pop!_OS / Ubuntu**: Install via [rustup](https://rustup.rs/), then `cargo install just`

System libraries:

- **Pop!_OS / Ubuntu / Debian**: `sudo apt install libdbus-1-dev libwayland-dev libxkbcommon-dev libssl-dev`
- **Fedora**: `sudo dnf install dbus-devel wayland-devel libxkbcommon-devel openssl-devel`
- **Arch Linux**: `sudo pacman -S dbus wayland libxkbcommon openssl`

#### Build and install

```bash
git clone https://github.com/marcossl10/cosmic-simple-feeds.git
cd cosmic-simple-feeds
just build-release
sudo just install
```

Log out and back in to refresh the panel icon cache.

Configuration
-------------

The applet stores configuration via `cosmic-config` at:

```
~/.config/cosmic/com.github.marcossl10.cosmic-simple-feeds/v3/
```

Each config field is a separate file in Ron (Rusty Object Notation) format:

| File | Format | Default |
|------|--------|---------|
| `feeds` | `["url1", "url2"]` | Google News RSS |
| `refresh_interval_seconds` | `300` | 5 minutes |

You can manage feeds from the popup UI (click the applet, then "Manage"),
or edit the config files directly. Changes are picked up live.

Development
-----------

### With Nix

```bash
nix develop    # enter dev shell with all dependencies
just run       # build and run the applet
cargo test     # run tests
cargo clippy   # lint
```

### Without Nix

Ensure system libraries are installed (see above), then:

```bash
just run       # build and run
cargo test     # run tests
```

License
-------

MIT
