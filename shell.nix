
{ nixpkgs ? (import <nixpkgs> {}) }:
let
  pkgs = nixpkgs;
  deps = with pkgs; [
    autoconf
    ocaml opam
    # ocamlPackages.findlib
    ocamlPackages.camlzip
    ocamlPackages.utop
    ocamlPackages.dune_3
  ];
  p = { }:
    pkgs.stdenv.mkDerivation rec {
        name = "worst";
        buildInputs = deps;
        enableSharedLibraries = false;
        enableSharedExecutables = false;
    };
in pkgs.callPackage p {}

