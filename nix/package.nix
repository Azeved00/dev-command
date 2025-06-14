{ pkgs ? import <nixpkgs> { }, ... }:

pkgs.rustPlatform.buildRustPackage {
    pname = "dev";
    version = "1.0.1";
    cargoLock.lockFile = ../Cargo.lock;
    src = pkgs.lib.cleanSource ../.;


    #install bash completion
    postInstall = ''
        #install -Dm644 target/release/build/my-rust-app/out/my-rust-app.bash \
        #    $out/share/bash-completion/completions/my-rust-app
    '';
}
