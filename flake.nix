{
  description = "competitive programming binaries";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    bundle-rs = {
      url = "github:chiaoicchi/bundle-rs";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      bundle-rs,
    }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
          (final: prev: {
            pythonPackagesExtensions = (prev.pythonPackagesExtensions or [ ]) ++ [
              (pyfinal: pyprev: {
                online-judge-api-client = pyprev.online-judge-api-client.overrideAttrs (old: {
                  patches = (old.patches or [ ]) ++ [ ./tools/patches/oj-api-atcoder-units.patch ];
                });
              })
            ];
          })
        ];
      };
      newTool = pkgs.writeShellApplication {
        name = "new";
        runtimeInputs = [
          pkgs.git
          pkgs.coreutils
          pkgs.gnused
        ];
        text = builtins.readFile ./tools/new.sh;
      };
      fetchTool = pkgs.writeShellApplication {
        name = "fetch";
        runtimeInputs = [
          pkgs.git
          pkgs.coreutils
          pkgs.jq
          pkgs.online-judge-tools
        ];
        text = builtins.readFile ./tools/fetch.sh;
      };
      ckTool = pkgs.writeShellApplication {
        name = "ck";
        runtimeInputs = [
          pkgs.git
          pkgs.coreutils
          pkgs.jq
          pkgs.online-judge-tools
        ];
        text = builtins.readFile ./tools/ck.sh;
      };
      bdTool = bundle-rs.packages.${system}.bd;

      commonPackages = [
        pkgs.online-judge-tools
        newTool
        fetchTool
        ckTool
        bdTool
      ];

      mkSiteShell =
        site:
        pkgs.mkShell {
          packages = [
            (pkgs.rust-bin.fromRustupToolchainFile (./. + "/${site}/rust-toolchain.toml"))
          ]
          ++ commonPackages;

          shellHook = ''
            root="$(git rev-parse --show-toplevel)"
            export CPLIB_ROOT="''${CPLIB_ROOT:-$root/../cplib-rs}"
            if [ ! -f "$CPLIB_ROOT/Cargo.toml" ]; then
              echo "warning: cplib-rs not found at $CPLIB_ROOT"
              echo "  git clone git@github.com:chiaoicchi/cplib-rs.git ~/src/cplib-rs"
              echo "  or set CPLIB_ROOT"
            fi
            echo "${site} environment"
            echo "  rust: $(rustc --version)"
            echo "  cplib: $CPLIB_ROOT"
          '';
        };
    in
    {
      packages.${system} = {
        new = newTool;
        fetch = fetchTool;
        ck = ckTool;
      };

      devShells.${system} = {
        atcoder = mkSiteShell "atcoder";
      };
    };
}
