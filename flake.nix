{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      };
      rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
        extensions = [ "rust-src" "rust-analyzer" ];
        targets = [ "wasm32-unknown-unknown" ];
      });
    in {
      devShells.x86_64-linux.default = pkgs.mkShell rec {
        nativeBuildInputs = with pkgs; [ rust pkg-config clang lld ];
        buildInputs = with pkgs; [ alsa-lib udev vulkan-loader libxkbcommon wayland libx11 libxcursor libxi libxrandr ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        WGPU_SETTINGS_PRIO = "webgpu";
      };
    };
}
