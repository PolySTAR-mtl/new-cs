
@echo off
setlocal EnableExtensions

title Installing Rust embedded tools

echo.
echo ==========================================
echo   Rust Embedded Tools Installer
echo ==========================================
echo.

REM ------------------------------------------
REM Check administrator-independent tools
REM ------------------------------------------

where rustup >nul 2>&1
if errorlevel 1 (
    echo [ERROR] rustup is not installed or not in PATH.
    echo Install Rust from https://rust-lang.org/tools/install/
    goto :error
)

where cargo >nul 2>&1
if errorlevel 1 (
    echo [ERROR] cargo is not available in PATH.
    goto :error
)

REM ------------------------------------------
REM Add embedded compilation target
REM ------------------------------------------

echo [1/3] Installing Rust target...

rustup target add thumbv7em-none-eabihf
if errorlevel 1 (
    echo [ERROR] Failed to install Rust target.
    goto :error
)

REM ------------------------------------------
REM Install cargo-binstall
REM ------------------------------------------

where cargo-binstall >nul 2>&1
if errorlevel 1 (
    echo [2/3] Installing cargo-binstall...

    powershell.exe -NoProfile -ExecutionPolicy Bypass -Command ^
        "Set-ExecutionPolicy Unrestricted -Scope Process -Force; iex (iwr 'https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1').Content"

    if errorlevel 1 (
        echo [ERROR] Failed to install cargo-binstall.
        goto :error
    )

    REM Refresh PATH for the current process
    set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
) else (
    echo [2/3] cargo-binstall already installed.
)

REM ------------------------------------------
REM Install probe-rs-tools
REM ------------------------------------------

echo [3/3] Installing probe-rs-tools...

cargo binstall --no-confirm probe-rs-tools
if errorlevel 1 (
    echo [ERROR] Failed to install probe-rs-tools.
    goto :error
)

echo.
echo ==========================================
echo   Installation completed successfully!
echo ==========================================
echo.

pause
exit /b 0

:error
echo.
echo Installation failed.
pause
exit /b 1
