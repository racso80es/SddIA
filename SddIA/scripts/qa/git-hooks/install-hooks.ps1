# Instala pre-commit Argos en .git/hooks/
$ErrorActionPreference = "Stop"
$RepoRoot = (git rev-parse --show-toplevel 2>$null)
if (-not $RepoRoot) {
    Write-Error "Ejecutar desde un repositorio git."
}
$Src = Join-Path $RepoRoot "SddIA\scripts\qa\git-hooks\pre-commit"
$DstDir = Join-Path $RepoRoot ".git\hooks"
$Dst = Join-Path $DstDir "pre-commit"
if (-not (Test-Path $Src)) {
    Write-Error "No se encuentra $Src"
}
New-Item -ItemType Directory -Force -Path $DstDir | Out-Null
Copy-Item -Force $Src $Dst
Write-Host "Instalado: $Dst"
Write-Host "Prueba: python SddIA/scripts/qa/git-hooks/pre_commit_gate.py"
