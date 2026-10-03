#----------------------------------------
# Compiler
# ---------------------------------------
@default_files = ('main.tex');
$pdflatex = 'pdflatex --shell-escape --halt-on-error -interaction=nonstopmode -file-line-error -synctex=1 %O %S';
$pdf_mode = 1;
$dvi_mode = 0;
$postscript_mode = 0;
$max_repeat = 10; 
@generated_exts = qw(aux bbl blg brf glg glo gls ist idx ilg ind lof log lot out toc fdb_latexmk fls xdv synctex.gz);
$pdf_previewer = 'zathura';
$pdf_update_method = 2;

#----------------------------------------
# Dependencies
#----------------------------------------
$bibtex_use = 2;
$bibtex = "biblatex";

# Glossaries #
add_cus_dep( 'glo', 'gls', 0, 'glo2gls' );
add_cus_dep( 'acn', 'acr', 0, 'glo2gls');
sub glo2gls {
    my $file = $_[0];
    $file =~ s{^out/}{};
    system("makeglossaries -d $out_dir $file") 
}

# makeindex 
if (scalar(@ist) > 0) {
    $makeindex = "makeindex -s $ist[0] %O -o %D %S";
}
