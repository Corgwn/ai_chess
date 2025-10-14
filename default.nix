{
  rustPlatform,
  glib,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  name = "ai_chess";
  src = ./.;
  buildInputs = [ glib ];
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
}
