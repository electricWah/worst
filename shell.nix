
{ nixpkgs ? (import <nixpkgs> {}) }:
let
  pkgs = nixpkgs;
  deps = with pkgs; [
    pkgsStatic.minizip
    pkg-config
    watchexec
    flamegraph
    python39Packages.gprof2dot
    graphviz
    linuxPackages.perf
    rustup
    wasm-pack
    openssl
    binaryen
    nodejs
    esbuild
    httplz
  ];
  p = { }:
    pkgs.stdenv.mkDerivation rec {
        name = "worst";
        buildInputs = deps;
        enableSharedLibraries = false;
        enableSharedExecutables = false;
    };
in pkgs.callPackage p {}

