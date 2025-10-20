{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    # fenix = {
    #   url = "github:nix-community/fenix";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
    # fenix,
    }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naerskLib = pkgs.callPackage naersk { };
      # fenixLib = fenix.packages."x86_64-linux";
      # rustToolchain = fenixLib.stable.toolchain;
    in
    {

      # (naerskLib.override {cargo=rustToolchain;rustc=rustToolchain;}).buildPackage
      packages."x86_64-linux".default = naerskLib.buildPackage {
        src = ./.;
        buildInputs = [
          pkgs.glib
          pkgs.openssl
          pkgs.sqlite
          pkgs.gcc
        ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };

      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = [
          pkgs.rustc
          pkgs.cargo
          pkgs.openssl
          pkgs.sqlite
          pkgs.gcc
          pkgs.fastchess
        ];

        nativeBuildInputs = [
          pkgs.pkg-config
        ];

        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          exec fish  
        '';
      };
    };
}
