let
  sources = import ./nix/sources.nix {};
  pkgs = import sources.nixpkgs {};

in
  pkgs.stdenv.mkDerivation {
    name = "adventofcode-2020";

    buildInputs = [
      pkgs.cargo
    ];

    src = ./.;

    buildPhase = ''
      cargo build
    '';

    installPhase = ''
      cp target/debug/adventofcode-2020 $out
    '';
  }
