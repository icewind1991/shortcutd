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
      overlays = [(import rust-overlay)];
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

      toolchain = pkgs.rust-bin.stable.latest.default.override {inherit targets;};
      assetNameForTarget = replaceStrings ["-unknown" "-gnu" "-musl" "eabihf" "-pc"] ["" "" "" "" ""];

      cross-naersk' = pkgs.callPackage cross-naersk {inherit naersk;};
      src = lib.sources.sourceByRegex (lib.cleanSource ./.) ["Cargo.*" "(server|client)(/.*)?"];
      naerskOpt = {
        pname = "shortcutd";
        root = src;
      };

      buildMatrix = targets: {
        include =
          map (target: {
            inherit target;
            artifact_name = "shortcutd";
            asset_name = "shortcutd-${assetNameForTarget target}";
          })
          targets
          ++ map (target: {
            target = "${target}-example-client";
            artifact_name = "client";
            asset_name = "example-client-${assetNameForTarget target}";
          })
          targets;
      };
      serverPackages = genAttrs targets (target:
        (cross-naersk' target).buildPackage {
          pname = "shortcutd";
          root = src;
          postInstall = ''
            mkdir -p $out/etc/dbus-1/system.d/
            cp ${./nixos-nl.icewind.shortcutd.conf} $out/etc/dbus-1/system.d/nl.icewind.shortcutd.conf
          '';
        });
      clientPackages = listToAttrs (map (target:
        nameValuePair "${target}-example-client" ((cross-naersk' target).buildPackage {
          pname = "shortcutd-example-client";
          root = src;

          overrideMain = x: {
            preConfigure = ''
              cargo_build_options="$cargo_build_options --example client"
            '';
          };
        }))
      targets);
    in rec {
      packages =
        serverPackages
        // clientPackages
        // rec {
          shortcutd = packages.${hostTarget};
          example-client = packages."${hostTarget}-example-client";
          check = (cross-naersk' hostTarget).buildPackage (naerskOpt
            // {
              mode = "check";
            });
          clippy = (cross-naersk' hostTarget).buildPackage (naerskOpt
            // {
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
    })
    // {
      nixosModule = {
        config,
        lib,
        pkgs,
        ...
      }:
        with lib; let
          cfg = config.services.shortcutd;
        in {
          options.services.shortcutd = {
            enable = mkEnableOption "Enables the shortcutd service";

            log = mkOption rec {
              type = types.str;
              default = "WARN";
              example = "INFO";
              description = "log level";
            };
          };

          config = mkIf cfg.enable {
            services.dbus.packages = [self.packages.${pkgs.system}.default];

            users.users.shortcutd = {
              isSystemUser = true;
              group = "shortcutd";
            };
            users.groups.shortcutd = {};

            systemd.services."shortcutd" = {
              wantedBy = ["multi-user.target"];

              environment = {
                RUST_LOG = cfg.log;
              };

              serviceConfig = let
                pkg = self.packages.${pkgs.system}.default;
              in {
                User = "shortcutd";
                Restart = "on-failure";
                ExecStart = "${pkg}/bin/shortcutd";
                PrivateTmp = true;
                ProtectSystem = "strict";
                ProtectHome = true;
                NoNewPrivileges = true;
                CapabilityBoundingSet = true;
                ProtectKernelLogs = true;
                ProtectControlGroups = true;
                SystemCallArchitectures = "native";
                ProtectKernelModules = true;
                RestrictNamespaces = true;
                MemoryDenyWriteExecute = true;
                ProtectHostname = true;
                LockPersonality = true;
                ProtectKernelTunables = true;
                RestrictRealtime = true;
                SystemCallFilter = ["@system-service" "~@resources" "~@privileged"];
                RestrictAddressFamilies = ["AF_UNIX"];
                IPAddressDeny = "any";
                PrivateUsers = true;
                RestrictSUIDSGID = true;
                PrivateNetwork = true;
                UMask = "0077";
                SupplementaryGroups = ["input"];
              };
            };
          };
        };
    };
}
