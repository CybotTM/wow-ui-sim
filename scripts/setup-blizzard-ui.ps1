# Populate the active profile's Blizzard UI cache from local WoW CASC data.
#
# Windows equivalent of setup-blizzard-ui.sh.
#
# Usage: powershell -ExecutionPolicy Bypass -File scripts\setup-blizzard-ui.ps1 [-Profile retail]

param(
    [string]$Profile = ""
)

$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path

if ($Profile -ne "") {
    Write-Host "Requested profile '$Profile'; wow-cli uses the active Cargo client feature for cache sync."
}

Push-Location $RepoRoot
try {
    cargo run --quiet --bin wow-cli -- casc sync-blizzard-ui
}
finally {
    Pop-Location
}
