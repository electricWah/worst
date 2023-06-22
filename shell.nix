
{ nixpkgs ? (import <nixpkgs> {}) }:
let
  pkgs = nixpkgs;
  deps = with pkgs; [
    # pkgsStatic.minizip
    pkg-config
    watchexec
    flamegraph
    # python39Packages.gprof2dot
    graphviz
    linuxPackages.perf
    # rustup
    openssl
    # binaryen
    # nodejs
    # esbuild
    # wasm-pack

    httplz

    janet
    # https://github.com/janet-lang/jpm/issues/68
    (pkgs.jpm.overrideAttrs (old: {
      buildInputs = old.buildInputs ++ [ pkgs.makeWrapper ];
      postInstall = "wrapProgram $out/bin/jpm --add-flags '--libpath=${pkgs.janet}/lib --ldflags=-L${pkgs.glibc}/lib --local'";
    }))
  ];
  p = { }:
    pkgs.stdenv.mkDerivation rec {
        name = "worst";
        buildInputs = deps;
        enableSharedLibraries = false;
        enableSharedExecutables = false;
        shellHook = ''
            export JANET_PATH=$(which janet)/..
            export JANET_TREE=$PWD/.jpm_tree
        '';
    };
in pkgs.callPackage p {}

