# Sparse-checkout Blizzard UI source from Gethe/wow-ui-source into vendor/,
# then junction Interface\BlizzardUI -> the checkout's Interface\AddOns.
#
# Windows equivalent of setup-blizzard-ui.sh. Junctions work without admin
# or Developer Mode; symbolic links would require both.
#
# Usage: powershell -ExecutionPolicy Bypass -File scripts\setup-blizzard-ui.ps1 [-Tag 12.0.5]

param(
    [string]$Tag = "12.0.5"
)

$ErrorActionPreference = "Stop"

function Invoke-Git {
    param([Parameter(Mandatory)][string[]]$Args)
    & git @Args
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Args -join ' ') failed (exit $LASTEXITCODE)"
    }
}

$RepoRoot  = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$VendorDir = Join-Path $RepoRoot 'vendor\wow-ui-source'
$Junction  = Join-Path $RepoRoot 'Interface\BlizzardUI'
$Target    = Join-Path $VendorDir 'Interface\AddOns'

if (Test-Path -LiteralPath $VendorDir) {
    Write-Host "Updating existing checkout to tag $Tag..."
    Push-Location $VendorDir
    try {
        & git fetch --depth=1 origin "refs/tags/${Tag}:refs/tags/${Tag}" 2>$null
        if ($LASTEXITCODE -ne 0) {
            Invoke-Git @('fetch', '--depth=1', 'origin', 'tag', $Tag)
        }
        Invoke-Git @('checkout', $Tag)
    }
    finally {
        Pop-Location
    }
}
else {
    Write-Host "Cloning wow-ui-source (sparse, tag $Tag)..."
    Invoke-Git @(
        'clone', '--filter=blob:none', '--no-checkout', '--depth=1',
        '--branch', $Tag,
        'https://github.com/Gethe/wow-ui-source.git', $VendorDir
    )
    Push-Location $VendorDir
    try {
        Invoke-Git @('sparse-checkout', 'init', '--cone')
        Invoke-Git @('sparse-checkout', 'set', 'Interface/AddOns')
        Invoke-Git @('checkout', $Tag)
    }
    finally {
        Pop-Location
    }
}

# Replace any existing junction/symlink at $Junction; refuse if it's a real dir.
if (Test-Path -LiteralPath $Junction) {
    $item = Get-Item -LiteralPath $Junction -Force
    if ($item.LinkType -in 'Junction', 'SymbolicLink') {
        Remove-Item -LiteralPath $Junction -Force
    }
    else {
        Write-Host "ERROR: $Junction is a real directory, not a junction. Remove it first." -ForegroundColor Red
        exit 1
    }
}

$junctionParent = Split-Path -Parent $Junction
if (-not (Test-Path -LiteralPath $junctionParent)) {
    New-Item -ItemType Directory -Path $junctionParent | Out-Null
}

New-Item -ItemType Junction -Path $Junction -Target $Target | Out-Null
Write-Host "Done: Interface\BlizzardUI -> vendor\wow-ui-source\Interface\AddOns (tag $Tag)"
