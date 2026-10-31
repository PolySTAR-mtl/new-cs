REM Install the required target with rustup (https://rust-lang.org/tools/install/)
rustup target add thumbv7em-none-eabihf
REM Install cargo binstall to avoid compiling probe-rs-tools with cargo install probe-rs-tools (https://github.com/cargo-bins/cargo-binstall)
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content
REM Install probe-rs-tools (https://probe.rs/docs/)
cargo binstall probe-rs-tools