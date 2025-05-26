{ config, lib, pkgs, ... }:
let
    cfg = config.programs.dev-command;
    package = import ./package.nix (pkgs);
    tomlFormat = pkgs.formats.toml { };
in
{
    options = {
        programs.dev-command = {
            enable = lib.mkEnableOption "Dev command";

            settings = lib.mkOption {
                type = tomlFormat.type;
                default = { };
                example = lib.literalExpression ''
                  {
                    window.dimensions = {
                      lines = 3;
                      columns = 200;
                    };
                    keyboard.bindings = [
                      {
                        key = "K";
                        mods = "Control";
                        chars = "\\u000c";
                      }
                    ];
                  }
                '';

                description = ''
                  Configuration written to
                  {file}`$XDG_CONFIG_HOME/alacritty/alacritty.yml` or
                  {file}`$XDG_CONFIG_HOME/alacritty/alacritty.toml`
                  (the latter being used for alacritty 0.13 and later).
                  See <https://github.com/alacritty/alacritty/tree/master#configuration>
                  for more info.
                '';
            };
        };
    };

    config = lib.mkIf cfg.enable {
        home.packages = [ package ];

        xdg.configFile."dev/config.toml" = lib.mkIf (cfg.settings != { }) {
            source = (tomlFormat.generate "config.toml" cfg.settings);
        };
    };
}
