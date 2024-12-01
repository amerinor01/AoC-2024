{ pkgs, nixpkgs, system, makeRustPlatform, rust-overlay }:
let
  common = {
    version = "0.0.1";
    src = ./.;

    cargoLock = {
      lockFile = ./Cargo.lock;
    };
    nativeBuildInputs = [ pkgs.pkg-config ];
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  };
in
{
  simulator = pkgs.rustPlatform.buildRustPackage (common // {
    pname = "simulator";
    cargoBuildFlags = "-p sim";
  });

}
