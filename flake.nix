{
  inputs = {
    nixpkgs.url = "nixpkgs/release-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:icewind1991/naersk?rev=6d245a3bbb2ee31ec726bb57b9a8b206302e7110";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    cross-naersk.url = "github:icewind1991/cross-naersk";
    cross-naersk.inputs.nixpkgs.follows = "nixpkgs";
    cross-naersk.inputs.naersk.follows = "naersk";
  };

  outputs = {
    self,
    flake-utils,
    cross-naersk,
    naersk,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = (import nixpkgs) {
        inherit system overlays;
      };
      lib = pkgs.lib;
      inherit (builtins) map replaceStrings;
      inherit (lib.strings) hasInfix;
      inherit (lib.attrsets) nameValuePair genAttrs listToAttrs;

      hostTarget = pkgs.hostPlatform.config;
      targets = [
        "x86_64-unknown-linux-musl"
        hostTarget
      ];

      releaseTargets = lib.lists.remove hostTarget targets;

      toolchain = (pkgs.rust-bin.stable.latest.default.override { inherit targets; });
      assetNameForTarget = replaceStrings ["-unknown" "-gnu" "-musl" "eabihf" "-pc"] ["" "" "" "" ""];

      cross-naersk' = pkgs.callPackage cross-naersk {inherit naersk;};
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(server|client)(/.*)?"];
      naerskOpt = {
        pname = "shortcutd";
        root = src;
      };

      buildMatrix = targets: {
        include = (map (target: {
          inherit target;
          artifact_name = "shortcutd";
          asset_name = "shortcutd-${assetNameForTarget target}";
        }) targets ++ map (target: {
          target = "${target}-example-client";
          artifact_name = "client";
          asset_name = "example-client-${assetNameForTarget target}";
        }) targets);
      };
      serverPackages = genAttrs targets (target: (cross-naersk' target).buildPackage ({
        pname = "shortcutd";
        root = src;
      }));
      clientPackages = listToAttrs (map (target: nameValuePair "${target}-example-client" ((cross-naersk' target).buildPackage ({
        pname = "shortcutd-example-client";
        root = src;

        overrideMain = x: {
            preConfigure = ''
            cargo_build_options="$cargo_build_options --example client"
          '';
        };
      }))) targets);
    in rec {
      packages = serverPackages // clientPackages // rec {
        shortcutd = packages.${hostTarget};
        example-client = packages."${hostTarget}-example-client";
        check = (cross-naersk' hostTarget).buildPackage (naerskOpt // {
          mode = "check";
        });
        clippy = (cross-naersk' hostTarget).buildPackage (naerskOpt // {
          mode = "clippy";
        });
        default = shortcutd;
      };

      inherit targets;
      inherit releaseTargets;
      matrix = buildMatrix targets;
      releaseMatrix = buildMatrix releaseTargets;

      apps = rec {
        tf-demo-parser = flake-utils.lib.mkApp {
          drv = packages.tf-demo-parser;
          exePath = "/bin/parse_demo";
        };
        default = tf-demo-parser;
      };

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [rust-bin.stable.latest.default bacon cargo-edit cargo-outdated rustfmt clippy cargo-audit hyperfine valgrind];
      };
    });
}
