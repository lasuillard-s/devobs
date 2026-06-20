{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # BUG: https://github.com/nixos/nixpkgs/issues/522307
        fixedPipx = pkgs.python3Packages.toPythonApplication (
          pkgs.python3Packages.pipx.overridePythonAttrs (oldAttrs: {
            doCheck = false;
          })
        );
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pre-commit
            just
            fixedPipx
            rustup
            cargo
            cargo-llvm-cov
            cargo-nextest
            cargo-watch
            cargo-insta
            maturin
          ];

          shellHook = ''
            pre-commit install
          '';
        };
      }
    );
}
