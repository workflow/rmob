{
  description = "Arr! Git mobbing in Rust";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        defaultPackage = with pkgs; rustPlatform.buildRustPackage rec {
          name = "rmob";
          src = ./.;

          cargoSha256 = "sha256-MSXofOklfy1t0vVEUko2rExgf7/lju7lLkjaOvlckqw=";

          buildInputs = [
            openssl
          ];

          doCheck = false; # Integration test currently depends on accessing the git directory

          # Needed to get openssl-sys to use pkg-config.
          # OPENSSL_NO_VENDOR = 1;

          nativeBuildInputs = lib.optionals stdenv.isLinux [ pkg-config ];

          meta = {
            description = "CLI swiss army knife fer shippin' code together";
            license = lib.licenses.gpl3;
          };
        };
      }
    );
}
