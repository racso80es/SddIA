$ErrorActionPreference = "Stop"
$RepoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..")).Path
$Targets = @(
    "SddIA_1/.SddIA",
    "SddIA_2/.SddIA",
    "SddIA_3/.SddIA",
    "SddIA_4/.SddIA",
    "SddIA/scripts/starter-kit/.SddIA"
)

Push-Location $RepoRoot
try {
    foreach ($rel in $Targets) {
        $dot = Join-Path $RepoRoot ($rel -replace "/", [IO.Path]::DirectorySeparatorChar)
        $legacy = Join-Path $dot "Tools"
        $canonical = Join-Path $dot "tools"
        if (-not (Test-Path -LiteralPath $legacy)) { continue }

        if (-not (Test-Path -LiteralPath $canonical)) {
            New-Item -ItemType Directory -Path $canonical -Force | Out-Null
        }

        $gitLegacy = ($rel -replace "\\", "/") + "/Tools"
        $gitCanon = ($rel -replace "\\", "/") + "/tools"
        foreach ($item in @(Get-ChildItem -LiteralPath $legacy -Force -ErrorAction SilentlyContinue)) {
            $dest = Join-Path $canonical $item.Name
            if ((Test-Path -LiteralPath $dest) -and $item.Name -eq "index.md") {
                [IO.File]::WriteAllBytes($dest, [IO.File]::ReadAllBytes($item.FullName))
            } elseif (-not (Test-Path -LiteralPath $dest)) {
                Move-Item -LiteralPath $item.FullName -Destination $dest -Force
            }
        }
        foreach ($g in @("$gitLegacy/index.md", $gitLegacy)) {
            git rm -rf --ignore-unmatch -- $g 2>$null | Out-Null
        }
        if (Test-Path -LiteralPath $canonical) {
            git add -f -- $gitCanon 2>$null | Out-Null
        }
        if (Test-Path -LiteralPath $legacy) {
            Remove-Item -LiteralPath $legacy -Recurse -Force -ErrorAction SilentlyContinue
        }
        Write-Output "OK $rel"
    }
} finally {
    Pop-Location
}
