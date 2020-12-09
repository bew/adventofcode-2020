let
  sources = import ./nix/sources.nix {};
  pkgs = import sources.nixpkgs {};

  dayDerivation = dayId: pkgs.stdenv.mkDerivation {
    name = "adventofcode-2020-${dayId}";

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
  };
in {
  day1 = dayDerivation "day01";
}
