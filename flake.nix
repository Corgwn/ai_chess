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
      perftree = pkgs.rustPlatform.buildRustPackage rec {
        pname = "perftree-cli";
        version = "0.2.0";
        src = pkgs.fetchFromGitHub {
          owner = "agausmann";
          repo = "perftree";
          rev = "423ceb0";
          hash = "sha256-s414KH34gJIKGit8A/Gg6v3JVVRApVnJz0skqeKplVg=";
        };
        cargoHash = "sha256-oSn9SF4EzjEzMg8q/bna/Fb3RlgGcNodqxd7hikYQx4=";
        doCheck = false;
      };
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
          pkgs.stockfish
          # perftree
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
