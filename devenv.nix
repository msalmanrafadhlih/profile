{ templateInputs }:
{
  pkgs,
  lib,
  config,
  ...
}:
let
  system = pkgs.stdenv.hostPlatform.system;
  fenix = templateInputs.fenix;
  cfg = config.setupRust;

  toolchain = fenix.packages.${system}.combine (
    [
      fenix.packages.${system}.stable.cargo
      fenix.packages.${system}.stable.rustc
      fenix.packages.${system}.stable.rust-src
      fenix.packages.${system}.stable.rust-analyzer
      fenix.packages.${system}.stable.clippy
      fenix.packages.${system}.stable.rustfmt
    ]
    ++ cfg.toolchains
  );
in
{
  options.setupRust = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = true;
      description = "Enable konfigurasi Rust untuk template ini.";
    };

    toolchains = lib.mkOption {
      type = lib.types.listOf lib.types.package;
      default = [ ];
      description = ''
        Komponen/target Rust tambahan (mis. fenix `targets.<target>.stable.rust-std`)
        yang digabung ke dalam SATU toolchain `combine` yang sama, bukan derivation terpisah.
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    packages = [
      toolchain
    ]
    ++ (with pkgs; [
      cargo-watch # auto-rebuild saat file berubah
      cargo-edit # cargo add/rm/upgrade
      cargo-audit # audit depedency
      pkg-config
      openssl
    ]);

    env.RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
    env.PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    enterShell = ''
      echo "🦀 Rust Dev Shell (${pkgs.stdenv.hostPlatform.system})"
      echo ""
      echo "cargo     : $(cargo --version 2>/dev/null | awk '{print $2}' || echo 'Not Found')"
      echo "rustc     : $(rustc --version 2>/dev/null | awk '{print $2}' || echo 'Not Found')"
      echo "targets   : $(rustc --print target-list 2>/dev/null | grep -c android || echo 0) android target(s) terpasang"
      echo ""
    '';
  };
}
