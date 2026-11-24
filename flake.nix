{
  description = "eyepiece-pc packages and development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.nixpkgs-darwin.url = "github:NixOS/nixpkgs/nixpkgs-26.05-darwin";

  outputs =
    { self, nixpkgs, nixpkgs-darwin, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forEachSystem = nixpkgs.lib.genAttrs systems;
      swiftLinkerFlags = pkgs: ''
        export NIX_LDFLAGS="$NIX_LDFLAGS -L$SDKROOT/usr/lib/swift -L${pkgs.swift}/lib/swift/macosx -lobjc"
      '';
    in
    {
      packages = forEachSystem (system:
        let
          pkgs = import nixpkgs { inherit system; };
          inherit (pkgs) lib stdenv;
          linuxLibraries = with pkgs; lib.optionals stdenv.hostPlatform.isLinux [
            libdrm
            pipewire
            wayland
            libxkbcommon
            libGL
            libx11
          ];
        in
        nixpkgs.lib.optionalAttrs (system != "x86_64-darwin") {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "eyepiece-pc";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;
            src = lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;
            dontUseSwiftpmBuild = true;
            dontUseSwiftpmCheck = true;
            dontUseSwiftpmInstall = true;

            nativeBuildInputs = with pkgs; [
              pkg-config
              rustPlatform.bindgenHook
            ] ++ lib.optionals stdenv.hostPlatform.isLinux [
              makeWrapper
            ] ++ lib.optionals stdenv.hostPlatform.isDarwin [
              swift
              swiftpm
            ];
            buildInputs = with pkgs; [ ffmpeg ] ++ linuxLibraries
              ++ lib.optionals stdenv.hostPlatform.isDarwin [ apple-sdk_26 ];

            FFMPEG_INCLUDE_DIR = "${pkgs.ffmpeg.dev}/include";
            FFMPEG_LIB_DIR = "${pkgs.ffmpeg.lib}/lib";

            preBuild = lib.optionalString stdenv.hostPlatform.isDarwin (swiftLinkerFlags pkgs);

            postFixup = lib.optionalString stdenv.hostPlatform.isLinux ''
              wrapProgram "$out/bin/eyepiece-gui" \
                --prefix LD_LIBRARY_PATH : "${lib.makeLibraryPath linuxLibraries}"
            '';

            meta = {
              description = "Stream PC screen and head tracking to DJI Goggles";
              license = lib.licenses.agpl3Plus;
              mainProgram = "eyepiece-pc";
              platforms = [ "aarch64-darwin" "aarch64-linux" "x86_64-linux" ];
            };
          };
        }
      );

      devShells = forEachSystem (
        system:
        let
          pkgs = import (if system == "x86_64-darwin" then nixpkgs-darwin else nixpkgs) {
            inherit system;
          };
          inherit (pkgs) lib;
          ffmpegDev = pkgs.ffmpeg.dev;
        in
        {
          default = pkgs.mkShell {
            inputsFrom = lib.optionals (system != "x86_64-darwin") [
              self.packages.${system}.default
            ];

            packages =
              with pkgs;
              [
                cargo
                clippy
                cmake
                ffmpeg
                git
                ninja
                rust-analyzer
                rustc
                rustfmt
                android-tools
              ]
              ++ lib.optionals (system == "x86_64-darwin") [
                pkg-config
                swift
                swiftpm
              ];

            FFMPEG_INCLUDE_DIR = "${ffmpegDev}/include";
            FFMPEG_LIB_DIR = "${pkgs.ffmpeg.lib}/lib";
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            shellHook = ''
              ${lib.optionalString (system == "aarch64-darwin") (swiftLinkerFlags pkgs)}
              export PKG_CONFIG_PATH="${
                lib.makeSearchPath "lib/pkgconfig" [ ffmpegDev ]
              }''${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
              echo "eyepiece-pc dev shell: cargo check --workspace"
            '';
          };
        }
      );
    };
}
