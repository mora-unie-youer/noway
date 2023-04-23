{ pkgs, ... }:

pkgs.devShell.mkShell {
  name = "noway";

  env = {
    RUST_BACKTRACE = "1";
  };

  packages = with pkgs; [
    # Toolchain required for C + Rust binaries building
    binutils
    gcc

    # Binaries and libraries needed for Rust crates
    eudev
    dbus
    libGL
    libinput
    libxkbcommon
    mesa
    pkg-config
    seatd
    wayland

    # Nightly Rust toolchain
    bacon
    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
      # Extensions which ease your development process
      extensions = [ "rust-analyzer" "rust-src" ];
    }))
  ];
}
