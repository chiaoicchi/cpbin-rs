{
  description = "competitive programming binaries";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
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
      bundleToolchain = pkgs.rust-bin.fromRustupToolchainFile ./tools/bundle-rs/rust-toolchain.toml;
      bundleRs =
        (pkgs.makeRustPlatform {
          cargo = bundleToolchain;
          rustc = bundleToolchain;
        }).buildRustPackage
          {
            pname = "bundle-rs";
            version = "0.1.0";
            src = ./tools/bundle-rs;
            cargoLock.lockFile = ./tools/bundle-rs/Cargo.lock;
          };
      bdTool = pkgs.writeShellApplication {
        name = "bd";
        runtimeInputs = [
          pkgs.git
          pkgs.coreutils
          pkgs.gnused
          pkgs.rustfmt
          bundleRs
          pkgs.wl-clipboard
        ];
        text = builtins.readFile ./tools/bd.sh;
      };

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
            if [ ! -d ../../cplib-rs ]; then
              echo "../../cplib-rs is not found:"
              echo "  git clone git@github.com:chiaoicchi/cplib-rs.git ~/src/cplib-rs"
            fi
            echo "${site} environment"
            echo "  rust: $(rustc --version)"
          '';
        };
    in
    {
      packages.${system} = {
        new = newTool;
        fetch = fetchTool;
        ck = ckTool;
        bd = bdTool;
        bundle-rs = bundleRs;
      };

      devShells.${system} = {
        atcoder = mkSiteShell "atcoder";

        bundle-rs = pkgs.mkShell {
          packages = [ bundleToolchain ];

          shellHook = ''
            echo "bundle-rs environment"
            echo "  rust: $(rustc --version)"
          '';
        };
      };
    };
}
