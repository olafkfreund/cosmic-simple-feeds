flake:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.cosmic-ext-applet-feeds;
in
{
  options.programs.cosmic-ext-applet-feeds = {
    enable = lib.mkEnableOption "Feeds — RSS reader applet for the COSMIC™ desktop";

    package = lib.mkPackageOption pkgs "cosmic-ext-applet-feeds" {
      default = flake.packages.${pkgs.stdenv.hostPlatform.system}.cosmic-ext-applet-feeds;
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
