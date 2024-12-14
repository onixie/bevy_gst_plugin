{
  description = "bevy + gstreamer development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
  let
    pkgs = nixpkgs.legacyPackages.${system};
    rust-toolchain = fenix.packages.${system}.fromToolchainFile {
      file = ./rust-toolchain.toml;
      sha256 = "sha256-s1RPtyvDGJaX/BisLT+ifVfuhDT1nZkZ1NcK8sbwELM=";
    };
    buildInputs = with pkgs; lib.lists.flatten [
      # for bevy
      systemd  # libudev
      alsa-lib # libasound
      ## x11
      (with xorg; [
        libX11 libXcursor libXi libxcb
      ])
      libxkbcommon
      ## gpu
      vulkan-loader
      glfw

      # for gstreamer
      gcc14Stdenv.cc.cc.lib # workaround to avoid GLIBC version mismatch
      (with gst_all_1; [
        glib
        gstreamer        # Tools like "gst-inspect", "gst-launch", etc.
        gst-plugins-base # Common plugins like "filesrc".
        gst-plugins-good # Specialized plugins separated by quality.
        gst-plugins-bad
        gst-plugins-ugly
        gst-libav # Plugin to reuse ffmpeg to play almost every video format.
        gst-vaapi # Plugin to Support the Video Audio (Hardware) Acceleration API.
      ])
    ];
  in with pkgs;{
    apps.rust-analyzer = {
      type = "app";
      program = "${rust-toolchain}/bin/rust-analyzer";
    };
    devShell = mkShell.override { stdenv = gcc14Stdenv; } {
      # dev dependencies
      inherit buildInputs;
      LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${
        lib.makeLibraryPath buildInputs
      }";
      # build tools
      nativeBuildInputs = [
        pkg-config
        rust-toolchain
      ];
      # dev tools
      packages = [
        emacs
        git
      ];
      shellHook = ''
          emacs -l ~/workdir/dot-emacs/init.el 1>/dev/null 2>&1 &
      '';
    };
  });
}
