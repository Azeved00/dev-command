{
    description = "Dev shell for latex writting";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, ... } @ inputs: 
    let
        system = "x86_64-linux";
        pkgs = import inputs.nixpkgs { inherit system; };

        tex = (pkgs.texlive.combine {
                inherit (pkgs.texlive) scheme-medium
                adjustbox titlesec csquotes notoccite wrapfig placeins 
                xcolor svg graphics caption nextpage biblatex footmisc
                amsmath commath braket mathpazo mathtools listings
                siunitx spverbatim epigraph  varwidth xargs transparent 
                algorithmicx hyperref url fancybox ragged2e multirow
                fontspec  glossaries bigfoot imakeidx enumitem catchfile
                systeme algorithm2e ifoddpage relsize ieeetran mathalpha
                semantic bold-extra xypic minted upquote comment environ
                totpages hyperxmp ifmtarg ncctools preprint
                cryptocode forloop pbox
                ;
        });
        buildLatexScript = pkgs.writeShellApplication {
            name = "build.sh";
            runtimeInputs = [tex pkgs.inkscape ];
            text = ''
                latexmk -outdir=out main.tex
            '';
        };

        watchLatexScript = pkgs.writeShellApplication {
            name = "watch.sh";
            runtimeInputs = [tex pkgs.inkscape ];
            text = ''
                latexmk -pvc -outdir=out main.tex
            '';
        };
 
    in
    {

        apps."${system}"= {
            default = {
                type = "app";
                program = "${watchLatexScript}/bin/watch.sh";
            };
            build = {
                type = "app";
                program = "${buildLatexScript}/bin/build.sh";
            };
        };

        devShells."${system}".default = pkgs.mkShell {
            buildInputs = [pkgs.texlab tex];
        };
    };
}
