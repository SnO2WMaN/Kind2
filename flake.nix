{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk";
    nix-flake.url = "github:numtide/nix-filter";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    nix-flake,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        naersk = pkgs.callPackage inputs.naersk {};
      in {
        packages.default = naersk.buildPackage {
          src = with nix-flake.lib;
            filter {
              root = ./.;
              include = [
                "Cargo.lock"
                "Cargo.toml"
                (inDirectory "src")
              ];
            };
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs = with pkgs; [
            openssl
          ];
        };
        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
          name = "kind2";
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            pkg-config
          ];
          buildInputs = with pkgs; [
            openssl
          ];
        };
      }
    );
}
