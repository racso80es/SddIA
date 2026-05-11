#requires -Version 5.1
# Migración: .SddIA/Tools/<tool>/spec.* → definiciones planas en la misma raíz física
# (.sddia/tools ≡ .SddIA/Tools en Windows) + scripts/tools/{name}/
# IMPORTANTE (Windows): no usar Remove-Item sobre la carpeta Tools completa tras escribir
# los .md planos; Tools y tools son la misma ruta. Solo se eliminan subcarpetas por tool.
# Uso: desde raíz del monorepo: powershell -File SddIA/scripts/migrate-local-tools-once.ps1
$ErrorActionPreference = 'Stop'
$root = Resolve-Path (Join-Path $PSScriptRoot '..\..')
$workspaces = @('SddIA_1', 'SddIA_2', 'SddIA_3', 'SddIA_4')

function Get-FrontMatterAndBody([string]$raw) {
  if ($raw -notmatch '(?s)^---\r?\n(.+?)\r?\n---\r?\n(.*)$') { return $null, $raw }
  return $Matches[1], $Matches[2]
}

function Process-FrontMatter([string]$fm, [string]$name) {
  $lines = $fm -split "`r?`n"
  $out = [System.Collections.Generic.List[string]]::new()
  foreach ($line in $lines) {
    if ($line -match '^\s*toolId\s*:') { continue }
    if ($line -match '^\s*implementation_path_ref\s*:') {
      [void]$out.Add("implementation_path_ref: scripts/tools/$name")
      continue
    }
    [void]$out.Add($line)
  }
  return ($out -join "`n")
}

function Extract-Objective([string]$body) {
  if ($body -notmatch '(?s)## Objetivo\s*\r?\n\r?\n(.+?)(?=\r?\n## |\z)') { return $null }
  return $Matches[1].Trim()
}

foreach ($ws in $workspaces) {
  $legacy = Join-Path $root "$ws/.SddIA/Tools"
  if (-not (Test-Path -LiteralPath $legacy)) {
    Write-Host "Omitido $ws (sin .SddIA/Tools)"
    continue
  }
  $toolDirs = @(Get-ChildItem -LiteralPath $legacy -Directory -ErrorAction SilentlyContinue)
  if ($toolDirs.Count -eq 0) {
    Write-Host "Omitido $ws (Tools sin subcarpetas; ya migrado?)"
    continue
  }

  $normativeData = [System.Collections.Generic.List[object]]::new()
  foreach ($dir in $toolDirs) {
    $name = $dir.Name
    $specMd = Join-Path $dir.FullName 'spec.md'
    if (-not (Test-Path -LiteralPath $specMd)) {
      Write-Warning "Sin spec.md: $name en $ws"
      continue
    }
    $raw = Get-Content -LiteralPath $specMd -Raw -Encoding UTF8
    $fm, $body = Get-FrontMatterAndBody $raw
    if ($null -eq $fm) {
      Write-Warning "Frontmatter inválido: $specMd"
      continue
    }
    $newFm = Process-FrontMatter $fm $name
    $body2 = $body -replace '\.SddIA/tool-capsules/', 'scripts/tools/'
    $body2 = $body2 -replace '\.SddIA/Tools/', '.sddia/tools/'
    $body2 = $body2 -replace '\*\*`\s*\.SddIA/tool-capsules/([^`]+)`\*\*', '**`scripts/tools/$1`**'
    $newMd = "---`n$newFm`n---`n$body2"
    $obj = Extract-Objective $body2
    if (-not $obj) { $obj = "Herramienta del proyecto ($name)." }
    $specJsonPath = Join-Path $dir.FullName 'spec.json'
    $j = $null
    if (Test-Path -LiteralPath $specJsonPath) {
      $j = Get-Content -LiteralPath $specJsonPath -Raw -Encoding UTF8 | ConvertFrom-Json
      $j | Add-Member -NotePropertyName 'name' -NotePropertyValue $name -Force
      if ($j.PSObject.Properties.Name -contains 'definition_path_ref') {
        $j.definition_path_ref = ".sddia/tools/$name"
      }
      if ($j.PSObject.Properties.Name -contains 'implementation_path_ref') {
        $j.implementation_path_ref = "scripts/tools/$name"
      }
    }
    [void]$normativeData.Add([pscustomobject]@{
        Name           = $name
        NormativeMd    = $newMd
        Objective      = $obj
        SpecJson       = $j
        SpecJsonExists = ($null -ne $j)
      })
  }

  foreach ($dir in $toolDirs) {
    Remove-Item -LiteralPath $dir.FullName -Recurse -Force
  }

  foreach ($item in $normativeData) {
    $name = $item.Name
    $outDef = Join-Path $legacy "$name.md"
    Set-Content -LiteralPath $outDef -Value $item.NormativeMd -Encoding UTF8

    $scriptsDir = Join-Path $root "$ws/scripts/tools/$name"
    New-Item -ItemType Directory -Path $scriptsDir -Force | Out-Null
    $consumer = @"
# $name

$($item.Objective)

## Cómo ejecutar

La implementación debe residir en la carpeta ``scripts/tools/$name/`` en la raíz de este workspace. Ejecute el launcher o binario que el equipo deposite allí (por ejemplo ``.bat`` o ``.exe`` en Windows). Los detalles de argumentos dependen de la cápsula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con ``success``, ``exitCode``, ``message``, ``result`` / feedback según aplique).
"@
    Set-Content -LiteralPath (Join-Path $scriptsDir "$name.md") -Value $consumer -Encoding UTF8

    if ($item.SpecJsonExists) {
      $item.SpecJson | ConvertTo-Json -Depth 10 | Set-Content -LiteralPath (Join-Path $scriptsDir 'spec.json') -Encoding UTF8
    }
  }

  $idxSrc = Join-Path $legacy 'index.md'
  if (Test-Path -LiteralPath $idxSrc) {
    $idxRaw = Get-Content -LiteralPath $idxSrc -Raw -Encoding UTF8
    $idxFm, $idxBody = Get-FrontMatterAndBody $idxRaw
    $idxFm2 = ($idxFm -split "`n") | ForEach-Object {
      if ($_ -match '^\s*definition_root\s*:') { 'definition_root: ".sddia/tools"' }
      elseif ($_ -match '^\s*delivery_root\s*:') { 'delivery_root: "scripts/tools"' }
      else { $_ }
    }
    $idxBody2 = $idxBody -replace '\.SddIA/Tools', '.sddia/tools' -replace '\.SddIA/tool-capsules', 'scripts/tools' -replace '\| toolId \|', '| name |'
    $newIdx = "---`n$($idxFm2 -join "`n")`n---`n$idxBody2"
    Set-Content -LiteralPath $idxSrc -Value $newIdx -Encoding UTF8
  }

  Write-Host "OK $ws"
}
Write-Host 'Migración completada.'
