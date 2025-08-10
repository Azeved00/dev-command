{ pkgs ? import <nixpkgs> { }, ... }:

pkgs.rustPlatform.buildRustPackage {
    pname = "dev";
    version = "1.0.3";
    cargoLock.lockFile = ../Cargo.lock;
    src = pkgs.lib.cleanSource ../.;


    #install bash completion
    postInstall = ''
        mkdir -p $out/share/bash-completion/completions
        cp extra/dev.bash $out/share/bash-completion/completions/
    '';
}
