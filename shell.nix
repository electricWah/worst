
{ nixpkgs ? (import <nixpkgs> {}) }:
let
  pkgs = nixpkgs;
  deps = with pkgs; [
    luajit # already has static
    luajitPackages.luarocks
    pkgsStatic.minizip
    pkg-config
    watchexec
  ];
  p = { }:
    pkgs.stdenv.mkDerivation rec {
        name = "lworsti";
        version = "0.0";
        buildInputs = deps;
        enableSharedLibraries = false;
        enableSharedExecutables = false;
    };
in pkgs.callPackage p {}


