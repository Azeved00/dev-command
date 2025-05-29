
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

        homeManagerModules.default = import ./nix/module.nix;

        devShells.${system}.default = pkgs.mkShell {
            inherit ROOT;
            name = "Dev";

            buildInputs = with pkgs; [
                cargo rustc
                rust-analyzer
            ];

            shellHook = ''
            '';
        };
    };
}

