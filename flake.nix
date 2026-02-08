{
  description = "RSS feed applet for the COSMIC Desktop Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
  }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        # Include i18n/ and resources/ alongside Rust sources
        srcFilter =
          path: type:
          (craneLib.filterCargoSources path type)
          || (builtins.match ".*\\.ftl$" path != null)
          || (builtins.match ".*\\.desktop$" path != null)
          || (builtins.match ".*\\.svg$" path != null)
          || (builtins.match ".*\\.xml$" path != null)
          || (builtins.match ".*i18n\\.toml$" path != null);

        cosmic-simple-feeds = craneLib.buildPackage {
          pname = "cosmic-simple-feeds";
          version = "0.1.0";

          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = srcFilter;
          };

          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
            just
          ];

          buildInputs = with pkgs; [
            dbus
            wayland
            libxkbcommon
            libinput
            mesa
            fontconfig
            freetype
            expat
            openssl
          ];

          # Let cargo handle the build, not just
          dontUseJustBuild = true;
          dontUseJustCheck = true;
          dontUseJustInstall = true;

          postInstall =
            let
              appid = "com.github-marcossl10.cosmic-simple-feeds";
            in
            ''
              install -Dm0644 resources/app.desktop \
                $out/share/applications/${appid}.desktop
              install -Dm0644 resources/app.metainfo.xml \
                $out/share/metainfo/${appid}.metainfo.xml
              install -Dm0644 resources/icons/hicolor/scalable/apps/${appid}.svg \
                $out/share/icons/hicolor/scalable/apps/${appid}.svg
              install -Dm0644 resources/icons/hicolor/scalable/apps/${appid}-symbolic.svg \
                $out/share/icons/hicolor/symbolic/apps/${appid}-symbolic.svg
            '';

          meta = with pkgs.lib; {
            description = "RSS feed applet for the COSMIC Desktop Environment";
            homepage = "https://github.com/marcossl10/cosmic-simple-feeds";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "feeds";
          };
        };
      in
      {
        packages = {
          inherit cosmic-simple-feeds;
          default = cosmic-simple-feeds;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            just
            pkg-config
            dbus
            wayland
            libxkbcommon
            libinput
            mesa
            fontconfig
            freetype
            expat
            openssl
            rust-analyzer
            clippy
            rustfmt
            nixd
            statix
            deadnix
          ];
        };
      }
    )
    // {
      overlays.default = final: _prev: {
        cosmic-simple-feeds = self.packages.${final.system}.cosmic-simple-feeds;
      };

      nixosModules.default = import ./nix/module.nix self;
      homeManagerModules.default = import ./nix/hm-module.nix self;
    };
}
