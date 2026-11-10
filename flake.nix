{
	description = "Rust embedded developpement.";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, flake-utils, fenix }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
				
				target = "thumbv7em-none-eabihf";

				rustToolchain = fenix.packages.${system}.combine [
					(fenix.packages.${system}.stable.withComponents [
						"cargo"
						"clippy"
						"rust-src"
						"rustc"
						"rustfmt"
						"rust-analyzer"
					])

					fenix.packages.${system}.targets.${target}.stable.rust-std
				];
			in {
				devShells.default = pkgs.mkShell {
          				packages = with pkgs; [
            					rustToolchain
					    	probe-rs-tools
					    	cargo-binutils
					    	llvmPackages.bintools
					    	pkg-config
					    	git
		    				just	
          				];

          				shellHook = ''
            					echo "Rust embedded development environment"
            					echo "Target: thumbv7em-none-eabihf"
            					echo "Probe-rs: $(probe-rs --version)"
          				'';

          				CARGO_BUILD_TARGET = target;
        			};
			}
		);
}
