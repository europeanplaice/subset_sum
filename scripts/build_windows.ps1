param(
    [string]$Interpreter = "python3.12",
    [string]$Target = "x86_64-pc-windows-msvc",
    [string]$OutputDir = "dist",
    [string[]]$ExtraArgs = @()
)

# Build a Windows wheel for CPython >=3.8 (abi3) using maturin.
$projectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Join-Path $projectRoot ".."
Set-Location $projectRoot

$maturinArgs = @(
    "build",
    "--release",
    "--features", "python",
    "--target", $Target,
    "--interpreter", $Interpreter,
    "--out", $OutputDir
) + $ExtraArgs

maturin @maturinArgs
