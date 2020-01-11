{ pkgs ? import <nixpkgs> {} }:

let
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;

in
pkgs.stdenv.mkDerivation rec {
  name = "nixpkgs-fmt";

  src = pkgs.fetchurl {
    url = "https://github.com/justinwoo/get-icon/releases/download/2020-01-11/get-icon";
    sha256 = "1ygsrdghlp1qirlvcr3xsxdxwp4zskc3jmflsxwl2y8rhwlnqbgs";
  };

  buildInputs = [ pkgs.makeWrapper ];

  dontStrip = true;

  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];

  unpackPhase = ''
    mkdir -p $out/bin
    TARGET=$out/bin/get-icon

    cp $src $TARGET
    chmod +x $TARGET

    patchelf $TARGET \
      --interpreter ${dynamic-linker} \
      --set-rpath ${libPath}
  '';

  dontInstall = true;
}
