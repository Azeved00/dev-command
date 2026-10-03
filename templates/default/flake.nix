{
    description = "";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, ... } @ inputs: 
    let
        system = "x86_64-linux";
        pkgs = import inputs.nixpkgs { inherit system; };
    in
    {
        devShells."${system}".default = pkgs.mkShell {
            buildInputs = [];
        };
    };
}
