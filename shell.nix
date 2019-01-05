let
    pkgs = import <nixpkgs> {};
    rust = pkgs.latest.rustChannels.stable.rust;
in
with pkgs;
stdenv.mkDerivation {
    name = "rs-shell";
    buildInputs = [
      rust
      pkgconfig
      openssl
      cmake
      csound
    ];
}


