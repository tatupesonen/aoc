{
  description = "AoC @tatupesonen";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Common packages
        commonPackages = with pkgs; [ git ];

        rustStable = pkgs.rust-bin.stable.latest.default;

        rustNightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.rustfmt);

      in {
        devShells = {
          aoc2022 = pkgs.mkShell {
            name = "aoc2022-shell";
            buildInputs = commonPackages ++ [ rustStable ];
            shellHook = ''
              echo "🔧 Entered devshell for aoc2022"
	      cd aoc2022/
            '';
          };

          aoc2023 = pkgs.mkShell {
            name = "aoc2023-shell";
            buildInputs = commonPackages ++ [ pkgs.beam.packages.erlangR25.elixir_1_14 ];
            shellHook = ''
              echo "🔧 Entered devshell for aoc2023 (Elixir)"
	      cd aoc2023/
            '';
          };

          aoc2024 = pkgs.mkShell {
            name = "aoc2024-shell";
            buildInputs = commonPackages ++ [ rustNightly ];
            shellHook = ''
              echo "🔧 Entered devshell for aoc2024 (Rust Nightly)"
	      cd aoc2024/
            '';
          };
        };
      });
}

