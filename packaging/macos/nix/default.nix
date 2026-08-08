{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "esprit";
  version = "0.1.0";

  src = lib.cleanSource ../../..;

  cargoLock = {
    lockFile = ../../../Cargo.lock;
  };

  meta = {
    description = "AI-powered local knowledge engine";
    homepage = "https://github.com/krtvysinghh/Esprit";
    license = lib.licenses.mit;
    platforms = lib.platforms.unix;
    mainProgram = "esprit";
  };
}
