
{
    description = "Dev Command";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs} : 
    let 
        system = "x86_64-linux";
        pkgs = import nixpkgs { inherit system; };
        package = import ./nix/package.nix (pkgs);
        ROOT = let p = builtins.getEnv "PWD"; in if p == "" then self else p;
    in
    {

        packages.${system}.default = package;

        templates = rec {
            latex = {
                path = ./templates/latex/.;
                description = "A general template for latex writting";
                welcomeText = ''
                    Latex template Loaded.

                    Run `nix develop` to enter the development shell, or
                    use the apps `build` and `watch` to directly build the document,
                    or rebuild on file changes
                '';
            };
            default = latex;
        };

        homeManagerModules.default = import ./nix/module.nix;

        devShells.${system}.default = pkgs.mkShell {
            inherit ROOT;
            name = "Dev";

            buildInputs = with pkgs; [
                cargo rustc
            ];

            shellHook = ''
            '';
        };
    };
}

