
{
    description = "Dev Command";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs} : 
    let 
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
        package = import ./package.nix (pkgs);
    in
    {

        packages.${system}.default = package;

        homeManagerModules.default = {lib, config, pkgs, ...}: with lib;
            let
                cfg = config.dev-command;
            in
            {
                imports = [ ];

                options.dev-command = {
                    enable = mkEnableOption "Enable Dev command";
                };

                config = mkIf cfg.enable {
                    environment.systemPackages = [ 
                        package
                    ];

                };
            };


        
    };
}

