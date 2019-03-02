let
    pkgs = import <nixpkgs> {};
    # rust = pkgs.latest.rustChannels.stable.rust;
in
with pkgs;
stdenv.mkDerivation {
    name = "rs-shell";
    buildInputs = [
      # rust
      rustup
      pkgconfig
      openssl
      cmake
      csound
      luajit
    ];
}


