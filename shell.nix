
{ nixpkgs ? (import <nixpkgs> {}) }:
let
  pkgs = nixpkgs;
  deps = with pkgs; [
    rustup
  ];
  p = { }:
    pkgs.stdenv.mkDerivation rec {
        name = "worst";
        buildInputs = deps;
        enableSharedLibraries = false;
        enableSharedExecutables = false;
    };
in pkgs.callPackage p {}

