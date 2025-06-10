{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
    in
    {
      devShells.${system}.default = with pkgs; mkShell.override {
        stdenv = stdenvAdapters.useMoldLinker clangStdenv;
      } rec {
        nativeBuildInputs = [ pkg-config ];
        buildInputs = [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          udev alsa-lib-with-plugins vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
          libxkbcommon wayland
        ];
        LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
      };
    };
}
