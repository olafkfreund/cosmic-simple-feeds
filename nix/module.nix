flake:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.cosmic-simple-feeds;
in
{
  options.programs.cosmic-simple-feeds = {
    enable = lib.mkEnableOption "COSMIC Simple Feeds RSS applet";

    package = lib.mkPackageOption pkgs "cosmic-simple-feeds" {
      default =
        if flake ? packages.${pkgs.stdenv.hostPlatform.system}.cosmic-simple-feeds then
          flake.packages.${pkgs.stdenv.hostPlatform.system}.cosmic-simple-feeds
        else
          pkgs.cosmic-simple-feeds;
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
