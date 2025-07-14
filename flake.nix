# C4/flake.nix
{
  description = "A development environment for the C4 Tauri application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05"; # Or a specific commit/branch if you prefer
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = {
            allowUnfree = true; # Needed for some codecs/components if Tauri pulls them in
          };
        };

        nodejs = pkgs.nodejs_22; # Explicitly use Node.js 22.x
        pnpm = pkgs.pnpm;

        gtkDeps = with pkgs; [
          gtk3 # Core GTK library
          webkitgtk # WebKitGTK engine
          libayatana-appindicator # For app indicators/tray icons
          librsvg # For SVG rendering
          gdk-pixbuf # For image loading
          glib # GLib utility library
          libsodium # Cryptography library
	  libsoup_3
	  haskellPackages.gi-javascriptcore
	  gdk-pixbuf
	  cairo
        ];

        # Database client libraries (e.g., for sqlx)
        dbClientDeps = with pkgs; [
          libpqxx # PostgreSQL client development headers (libpq)
          # Add other database clients here if needed, e.g., sqlite
          # sqlite.dev # For SQLite development headers
        ];

      in
      {
        devShell = pkgs.mkShell {
          name = "c4-tauri-dev-shell";

          packages = [
            nodejs
            pnpm
          ] ++ gtkDeps ++ dbClientDeps;

          # Environment variables and commands that run when you enter the shell
          shellHook = ''
            echo "Welcome to the C4 Tauri development environment!"
            echo "Rust version: $(rustc --version)"
            echo "Node.js version: $(node --version)"
            echo "pnpm version: $(pnpm --version)"
            export CARGO_HOME=$HOME/.cargo # Ensure cargo uses your home directory for caches/binaries
            export NPM_CONFIG_PREFIX=$HOME/.npm-global # Optional: for global npm packages if you use them
          '';

          # Optional: Specify dependencies for your specific Rust crates here if needed,
          # though they are usually handled by Cargo.toml itself.
          # We primarily use this for system-level dependencies.
        };

        # You could also define packages to build your backend/frontend here:
        # packages.c4Backend = pkgs.callPackage ./backend { };
        # packages.c4Frontend = pkgs.callPackage ./frontend { };
      });
}
