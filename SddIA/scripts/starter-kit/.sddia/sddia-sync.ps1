#requires -Version 5.1
<#
  Contrato de sincronización del motor SddIA (clone shallow en ./SddIA).

  ZONA SAGRADA — NO TOCAR desde este script:
    .sddia/tools/
  Las definiciones normativas locales del cliente viven ahí (evolution c3a9f1b2-8e4d-42c6-a7d3-9f0e1b2c3d4a).
  Este script solo elimina y reclona el árbol "SddIA/" bajo la raíz del repo; cualquier futura
  copia/merge adicional DEBE excluir explícitamente .sddia/tools/
#>
$ErrorActionPreference = 'Stop'

function Get-SddiaConfig {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Path
  )
  if (-not (Test-Path -LiteralPath $Path)) {
    throw "No existe el archivo requerido: $Path"
  }
  $raw = Get-Content -LiteralPath $Path -Raw
  try {
    return ($raw | ConvertFrom-Json)
  } catch {
    throw "JSON inválido en: $Path"
  }
}

function Ensure-GitIgnoreHasSddIA {
  param(
    [Parameter(Mandatory = $true)]
    [string]$RepoRoot
  )
  $gitIgnorePath = Join-Path $RepoRoot '.gitignore'
  $entry = 'SddIA/'

  if (-not (Test-Path -LiteralPath $gitIgnorePath)) {
    Set-Content -LiteralPath $gitIgnorePath -Value @($entry) -Encoding UTF8
    return
  }

  $lines = Get-Content -LiteralPath $gitIgnorePath
  if ($lines -notcontains $entry) {
    Add-Content -LiteralPath $gitIgnorePath -Value $entry
  }
}

function Remove-IfExists {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Path
  )
  if (Test-Path -LiteralPath $Path) {
    Remove-Item -LiteralPath $Path -Recurse -Force
  }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$sddiaDir = Split-Path -Parent $scriptDir
$repoRoot = Split-Path -Parent $sddiaDir

$configPath = Join-Path $scriptDir 'sddia-version.json'
$config = Get-SddiaConfig -Path $configPath

if (-not $config.repo_url -or -not $config.target_version) {
  throw "Faltan claves obligatorias en sddia-version.json: repo_url y/o target_version"
}

$repoUrl = [string]$config.repo_url
$targetVersion = [string]$config.target_version

$dest = Join-Path $repoRoot 'SddIA'

Remove-IfExists -Path $dest

git clone --depth 1 --branch $targetVersion $repoUrl $dest
if ($LASTEXITCODE -ne 0) { throw "git clone falló (exit_code=$LASTEXITCODE)" }

Remove-IfExists -Path (Join-Path $dest '.git')

Ensure-GitIgnoreHasSddIA -RepoRoot $repoRoot

Write-Host "SddIA inyectado en: $dest"
