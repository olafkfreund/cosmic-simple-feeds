flake:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.cosmic-ext-applet-feeds;

  # COSMIC config stores each field as a separate file in Ron format.
  # App ID (dots): io.github.olafkfreund.cosmic-ext-applet-feeds
  # Config version: 3
  configDir = "cosmic/io.github.olafkfreund.cosmic-ext-applet-feeds/v3";

  # Serialize a list of strings to Ron format: ["url1", "url2"]
  feedsToRon =
    feeds:
    "[${lib.concatMapStringsSep ", " (f: ''"${f}"'') feeds}]";
in
{
  options.programs.cosmic-ext-applet-feeds = {
    enable = lib.mkEnableOption "Feeds — RSS reader applet for the COSMIC™ desktop";

    package = lib.mkPackageOption pkgs "cosmic-ext-applet-feeds" {
      default = flake.packages.${pkgs.stdenv.hostPlatform.system}.cosmic-ext-applet-feeds;
    };

    feeds = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [ ];
      example = [
        "https://planet.nixos.org/rss20.xml"
        "https://weekly.nixos.org/feeds/all.rss.xml"
      ];
      description = ''
        List of RSS feed URLs to subscribe to.
        Written to COSMIC config in Ron format.
      '';
    };

    refreshInterval = lib.mkOption {
      type = lib.types.ints.positive;
      default = 300;
      description = ''
        Feed refresh interval in seconds.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile = lib.mkMerge [
      (lib.mkIf (cfg.feeds != [ ]) {
        "${configDir}/feeds".text = feedsToRon cfg.feeds;
      })
      {
        "${configDir}/refresh_interval_seconds".text = toString cfg.refreshInterval;
      }
    ];
  };
}
