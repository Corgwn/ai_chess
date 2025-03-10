let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rustChannel = pkgs.rustChannelOf { channel = "stable"; };
  rust = (rustChannel.rust.override {
    targets = [
      "wasm32-unknown-unknown" # required for the web-app
      "x86_64-unknown-linux-gnu"
    ];
    extensions = [ "rust-src" "rust-analysis" ];
  });

  # required for the desktop app
  runtime_deps = with pkgs;
    [ libGL ] ++ (with pkgs.xorg; [ libX11 libXcursor libXrandr libXi ]);

in with pkgs;
mkShell {
  buildInputs = [

    rust

    # required for the web-app
    sassc

    # required for the desktop
    freetype
    expat
    fontconfig
    flamegraph
  ];

  LD_LIBRARY_PATH = "${lib.makeLibraryPath runtime_deps}";
}
