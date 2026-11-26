{
  # A development shell only: it provides the toolchain and libraries, and the build is the
  # regular one (`cargo build --release`, `pnpm build` in web/), as on every other platform.
  description = "eyepiece-pc development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.nixpkgs-darwin.url = "github:NixOS/nixpkgs/nixpkgs-26.05-darwin";

  outputs =
    { nixpkgs, nixpkgs-darwin, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      devShells = nixpkgs.lib.genAttrs systems (
        system:
        let
          pkgs = import (if system == "x86_64-darwin" then nixpkgs-darwin else nixpkgs) {
            inherit system;
          };
          inherit (pkgs) lib stdenv;
          ffmpegDev = pkgs.ffmpeg.dev;
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
              rustPlatform.bindgenHook # libclang and the C headers for the FFmpeg bindings
            ];

            buildInputs =
              with pkgs;
              [ ffmpeg ]
              ++ lib.optionals stdenv.hostPlatform.isLinux [
                libdrm
                pipewire
                wayland
                libxkbcommon
                libGL
                libx11
              ]
              ++ lib.optionals (stdenv.hostPlatform.isDarwin && pkgs ? apple-sdk_26) [ apple-sdk_26 ];

            packages =
              with pkgs;
              [
                cargo
                clippy
                rustc
                rustfmt
                rust-analyzer
                nodejs_24 # web UI
                pnpm
                android-tools # adb
                git
              ]
              ++ lib.optionals stdenv.hostPlatform.isDarwin [
                swift
                swiftpm
              ];

            FFMPEG_INCLUDE_DIR = "${ffmpegDev}/include";
            FFMPEG_LIB_DIR = "${pkgs.ffmpeg.lib}/lib";
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            shellHook = ''
              ${lib.optionalString stdenv.hostPlatform.isDarwin ''
                export NIX_LDFLAGS="$NIX_LDFLAGS -L$SDKROOT/usr/lib/swift -L${pkgs.swift}/lib/swift/macosx -lobjc"
              ''}
              echo "eyepiece-pc dev shell: cargo build --release; (cd web && pnpm install && pnpm build)"
            '';
          };
        }
      );
    };
}
