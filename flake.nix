{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    git-hooks.url = "github:cachix/git-hooks.nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      advisory-db,
      git-hooks,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        craneLib = crane.mkLib nixpkgs.legacyPackages.${system};
        src = craneLib.cleanCargoSource ./.;

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        buildInputs = with pkgs; [ SDL2_ttf ];

        nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          # Additional darwin specific inputs can be set here
          pkgs.gcc
          pkgs.libiconv
        ];

        # Build *just* the cargo dependencies, so we can reuse all of that work when running in CI.
        cargoArtifacts = craneLib.buildDepsOnly { inherit src nativeBuildInputs; };

        # Build the app itself, reusing the dependency artifacts from above.
        qanata = craneLib.buildPackage {
          inherit
            cargoArtifacts
            src
            buildInputs
            nativeBuildInputs
            ;
          doCheck = false;
          meta = with pkgs.lib; {
            description = "Application aware layer switching with kanata and sway";
            homepage = "https://github.com/veyxov/qanata";
            maintainers = [ maintainers.lafrenierejm ];
            license = licenses.gpl3;
          };
        };
      in
      rec {
        packages = flake-utils.lib.flattenTree {
          # `nix build .#qanata`
          inherit qanata;
          # `nix build`
          default = qanata;
        };

        # `nix run`
        apps.default = flake-utils.lib.mkApp { drv = packages.qanata; };

        # `nix flake check`
        checks =
          {
            audit = craneLib.cargoAudit { inherit src advisory-db; };

            clippy = craneLib.cargoClippy {
              inherit cargoArtifacts src nativeBuildInputs;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            };

            doc = craneLib.cargoDoc { inherit cargoArtifacts src; };

            fmt = craneLib.cargoFmt { inherit src; };

            nextest = craneLib.cargoNextest {
              inherit cargoArtifacts src nativeBuildInputs;
              partitions = 1;
              partitionType = "count";
            };

            pre-commit = git-hooks.lib."${system}".run {
              src = ./.;
              hooks = {
                nixfmt-rfc-style.enable = true;
                rustfmt.enable = true;
                typos.enable = true;
              };
            };
          }
          // pkgs.lib.optionalAttrs (system == "x86_64-linux") {
            # NB: cargo-tarpaulin only supports x86_64 systems
            # Check code coverage (note: this will not upload coverage anywhere)
            qanata-coverage = craneLib.cargoTarpaulin { inherit cargoArtifacts src; };
          };

        # `nix develop`
        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.pre-commit) shellHook;
          inherit buildInputs;
          inputsFrom = builtins.attrValues self.checks;
          packages = with pkgs; [
            cargo
            clippy
            rustc
          ];
          nativeBuildInputs =
            nativeBuildInputs ++ (with pkgs; lib.optionals (system == "x86_64-linux") [ cargo-tarpaulin ]);
        };
      }
    );
}
