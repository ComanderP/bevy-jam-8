{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      devShells.x86_64-linux.default = pkgs.mkShell rec {
        nativeBuildInputs = with pkgs; [ cargo rustc pkg-config lld clippy rustfmt ];
        buildInputs = with pkgs; [ alsa-lib udev vulkan-loader libxkbcommon wayland libx11 libxcursor libxi libxrandr ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    };
}
