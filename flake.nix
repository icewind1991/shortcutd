{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:icewind1991/naersk?rev=6d245a3bbb2ee31ec726bb57b9a8b206302e7110";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = (import nixpkgs) {
        inherit system overlays;
      };
      lib = pkgs.lib;
      naerskForTarget = target: let
        toolchain = pkgs.rust-bin.stable.latest.default.override { targets = [target]; };
      in pkgs.callPackage naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
      hostTarget = pkgs.hostPlatform.config;
      targets = ["x86_64-pc-windows-gnu" hostTarget];
      hostNaersk = naerskForTarget hostTarget;
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(src)(/.*)?"];
      nearskOpt = {
        pname = "vbspview";
        root = src;
        nativeBuildInputs = buildDependencies;
      };
      crossArgs = {
        "x86_64-pc-windows-gnu" = {
          nativeBuildInputs = [ pkgs.pkgsCross.mingwW64.stdenv.cc ];
          overrideMain = args: args // { buildInputs = [ pkgs.pkgsCross.mingwW64.windows.pthreads ]; };
        };
      };
      crossArgsFor = target: if (hostTarget != target) then (crossArgs.${target} or {}) else {};
      buildDependencies = with pkgs; [
        freetype
        pkgconfig
        dbus
        libGL
      ];
    in rec {
      packages = (lib.attrsets.genAttrs targets (target: (naerskForTarget target).buildPackage (nearskOpt // {
        CARGO_BUILD_TARGET = target;
      } // (crossArgsFor target)))) // rec {
        vbspview = packages.${hostTarget};
        check = hostNaersk.buildPackage (nearskOpt // {
          mode = "check";
        });
        clippy = hostNaersk.buildPackage (nearskOpt // {
          mode = "clippy";
        });
        default = vbspview;
      };

      inherit targets;

      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          pkgs.rust-bin.stable.latest.default
          bacon
          cargo-edit
          cargo-outdated
          clippy
          cargo-audit
          cargo-msrv
        ] ++ buildDependencies;

        LD_LIBRARY_PATH = with pkgs; "/run/opengl-driver/lib/:${lib.makeLibraryPath ([libGL libGLU])}";
      };
    });
}
