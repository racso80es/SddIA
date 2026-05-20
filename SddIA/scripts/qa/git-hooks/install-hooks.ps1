# Instala hooks SddIA dinámicamente en .git/hooks/ (O5)
$ErrorActionPreference = "Stop"
$RepoRoot = (git rev-parse --show-toplevel 2>$null)
if (-not $RepoRoot) {
    Write-Error "Ejecutar desde un repositorio git."
}
$SrcDir = Join-Path $RepoRoot "SddIA\scripts\qa\git-hooks"
$DstDir = Join-Path $RepoRoot ".git\hooks"
$ExcludeExt = @(".py", ".ps1", ".sh", ".md", ".json", ".txt")

if (-not (Test-Path $SrcDir)) {
    Write-Error "No se encuentra $SrcDir"
}
New-Item -ItemType Directory -Force -Path $DstDir | Out-Null

$installed = @()
Get-ChildItem -File $SrcDir | ForEach-Object {
    if ($ExcludeExt -contains $_.Extension.ToLower()) { return }
    if ($_.BaseName -like "install-hooks*") { return }
    if ($_.Extension) { return }
    $dst = Join-Path $DstDir $_.Name
    Copy-Item -Force $_.FullName $dst
    $installed += $_.Name
}

if ($installed.Count -eq 0) {
    Write-Warning "No se encontraron hooks instalables en $SrcDir"
} else {
    Write-Host "Hooks instalados en $DstDir :"
    $installed | ForEach-Object { Write-Host "  - $_" }
}
