# KERNEL RAW — ejecución TEKTON (Windows PowerShell)
$ErrorActionPreference = 'Stop'
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot '..\..\..') | Select-Object -ExpandProperty Path
$Core = Join-Path $RepoRoot 'SddIA'
$limboArchetype = Join-Path $Core 'scripts\limbo\archetypes'
$starterBase = Join-Path $Core 'scripts\starter-kit\.sddia'

function Ensure-Dir([string]$p) {
    if (-not (Test-Path -LiteralPath $p)) { New-Item -ItemType Directory -Path $p -Force | Out-Null }
}

$coreNormsInjected = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$skippedProtected = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::OrdinalIgnoreCase)
$normsFilesMoved = 0
$principlesFoldersMoved = 0
$patternsFoldersMoved = 0

# --- PASO 0 ---
$normsRoot = Join-Path $Core 'norms'
$secRoot = Join-Path $Core 'security'
$protected = @{}
Get-ChildItem -LiteralPath $normsRoot -File -ErrorAction SilentlyContinue | ForEach-Object { $protected[$_.BaseName] = $true }
Get-ChildItem -LiteralPath $secRoot -File -ErrorAction SilentlyContinue | ForEach-Object { $protected[$_.BaseName] = $true }

$coreNormStems = @(
    'obediencia-procesos', 'capsule-json-io', 'commands-via-skills-or-tools', 'paths-via-cumulo',
    'entidades-dominio-ecosistema-sddia', 'git-via-skills-or-process', 'agents-principles-contract',
    'touchpoints-ia', 'sddia-evolution-sync'
)
$starterNormStems = @(
    'features-documentation-pattern', 'pr-acceptance-protocol', 'openapi-contract-rest-frontend', 'nextjs-hydration-client-state'
)

$claimedCoreNorm = @{}
$claimedStarterNorm = @{}

# --- PASO 1 ---
foreach ($n in @(4, 3, 2, 1)) {
    $normDir = Join-Path $RepoRoot "SddIA_$n\norms"
    if (-not (Test-Path -LiteralPath $normDir)) { continue }
    $destLimbo = Join-Path $limboArchetype "norms\SddIA_$n"
    foreach ($f in Get-ChildItem -LiteralPath $normDir -File | Sort-Object Name) {
        $stem = $f.BaseName
        Ensure-Dir $destLimbo

        if ($stem -eq 'features-documentation-frontmatter') {
            Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
            continue
        }
        if ($protected.ContainsKey($stem)) {
            Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
            [void]$skippedProtected.Add($stem)
            continue
        }
        if ($starterNormStems -contains $stem) {
            if ($claimedStarterNorm.ContainsKey($stem)) {
                Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
            } else {
                Ensure-Dir (Join-Path $starterBase 'norms')
                $target = Join-Path $starterBase "norms\$($f.Name)"
                if (Test-Path -LiteralPath $target) {
                    Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
                } else {
                    Move-Item -LiteralPath $f.FullName -Destination $target -Force
                    $claimedStarterNorm[$stem] = $true
                    $normsFilesMoved++
                }
            }
            continue
        }
        if ($coreNormStems -contains $stem) {
            $key = if ($stem -eq 'obediencia-procesos') { "$stem$($f.Extension.ToLowerInvariant())" } else { $stem }
            if ($claimedCoreNorm.ContainsKey($key)) {
                Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
            } else {
                $target = Join-Path $normsRoot $f.Name
                if (Test-Path -LiteralPath $target) {
                    Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
                } else {
                    Move-Item -LiteralPath $f.FullName -Destination $target -Force
                    $claimedCoreNorm[$key] = $true
                    [void]$coreNormsInjected.Add($stem)
                }
            }
            continue
        }
        Move-Item -LiteralPath $f.FullName -Destination (Join-Path $destLimbo $f.Name) -Force
    }
}

function Get-NatureFromSpec([string]$specPath) {
    if (-not (Test-Path -LiteralPath $specPath)) { return 'absent' }
    try {
        $j = Get-Content -LiteralPath $specPath -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($null -eq $j.nature -or [string]::IsNullOrWhiteSpace([string]$j.nature)) { return 'absent' }
        return [string]$j.nature
    } catch {
        return 'absent'
    }
}

function Get-SecurityRouting([string]$specPath) {
    if (-not (Test-Path -LiteralPath $specPath)) { return 'product' }
    try {
        $j = Get-Content -LiteralPath $specPath -Raw -Encoding UTF8 | ConvertFrom-Json
        $n = $j.nature
        if ($null -ne $n -and -not [string]::IsNullOrWhiteSpace([string]$n)) {
            if ([string]$n -eq 'motor') { return 'motor' }
            return 'product'
        }
        $ctx = $j.context
        if ($null -ne $ctx -and [string]$ctx -eq 'sddia-core') { return 'motor' }
        return 'product'
    } catch {
        return 'product'
    }
}

$claimedPrinciple = @{}
$claimedPattern = @{}
$claimedSecUuid = @{}

# --- PASO 2 principles ---
foreach ($n in @(4, 3, 2, 1)) {
    $prDir = Join-Path $RepoRoot "SddIA_$n\principles"
    if (-not (Test-Path -LiteralPath $prDir)) { continue }
    $limboPr = Join-Path $limboArchetype "principles\SddIA_$n"
    foreach ($f in Get-ChildItem -LiteralPath $prDir -File | Sort-Object Name) {
        Ensure-Dir $limboPr
        if ($f.Name -eq 'README.md' -or $f.BaseName -eq 'principles-contract') {
            Move-Item -LiteralPath $f.FullName -Destination (Join-Path $limboPr $f.Name) -Force
        }
    }
    foreach ($d in Get-ChildItem -LiteralPath $prDir -Directory | Sort-Object Name) {
        $name = $d.Name
        $limboEnt = Join-Path $limboPr $name
        $coreDest = Join-Path $Core "principles\$name"
        $starterDest = Join-Path $starterBase "principles\$name"
        Ensure-Dir (Join-Path $starterBase 'principles')

        if (Test-Path -LiteralPath $coreDest) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            continue
        }

        $specJson = Join-Path $d.FullName 'spec.json'
        $nat = Get-NatureFromSpec $specJson
        $isMotor = ($nat -eq 'motor')

        if ($isMotor) {
            Move-Item -LiteralPath $d.FullName -Destination $coreDest -Force
            continue
        }

        # product o absent -> starter kit
        if ($claimedPrinciple.ContainsKey($name)) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
        } else {
            if (Test-Path -LiteralPath $starterDest) {
                Ensure-Dir (Split-Path $limboEnt -Parent)
                Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            } else {
                Move-Item -LiteralPath $d.FullName -Destination $starterDest -Force
                $claimedPrinciple[$name] = $true
                $principlesFoldersMoved++
            }
        }
    }
}

# --- PASO 2 patterns ---
foreach ($n in @(4, 3, 2, 1)) {
    $patDir = Join-Path $RepoRoot "SddIA_$n\patterns"
    if (-not (Test-Path -LiteralPath $patDir)) { continue }
    $limboPa = Join-Path $limboArchetype "patterns\SddIA_$n"
    foreach ($f in Get-ChildItem -LiteralPath $patDir -File | Sort-Object Name) {
        Ensure-Dir $limboPa
        if ($f.Name -eq 'README.md' -or $f.BaseName -eq 'patterns-contract') {
            Move-Item -LiteralPath $f.FullName -Destination (Join-Path $limboPa $f.Name) -Force
        }
    }
    foreach ($d in Get-ChildItem -LiteralPath $patDir -Directory | Sort-Object Name) {
        $name = $d.Name
        $limboEnt = Join-Path $limboPa $name
        $coreDest = Join-Path $Core "patterns\$name"
        $starterDest = Join-Path $starterBase "patterns\$name"
        Ensure-Dir (Join-Path $starterBase 'patterns')

        if (Test-Path -LiteralPath $coreDest) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            continue
        }

        $specJson = Join-Path $d.FullName 'spec.json'
        $nat = Get-NatureFromSpec $specJson
        $isMotor = ($nat -eq 'motor')

        if ($isMotor) {
            Move-Item -LiteralPath $d.FullName -Destination $coreDest -Force
            continue
        }

        if ($claimedPattern.ContainsKey($name)) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
        } else {
            if (Test-Path -LiteralPath $starterDest) {
                Ensure-Dir (Split-Path $limboEnt -Parent)
                Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            } else {
                Move-Item -LiteralPath $d.FullName -Destination $starterDest -Force
                $claimedPattern[$name] = $true
                $patternsFoldersMoved++
            }
        }
    }
}

# --- PASO 3 security ---
foreach ($n in @(4, 3, 2, 1)) {
    $secDir = Join-Path $RepoRoot "SddIA_$n\security"
    if (-not (Test-Path -LiteralPath $secDir)) { continue }
    $limboSe = Join-Path $limboArchetype "security\SddIA_$n"
    foreach ($f in Get-ChildItem -LiteralPath $secDir -File | Sort-Object Name) {
        Ensure-Dir $limboSe
        $stem = $f.BaseName
        if ($protected.ContainsKey($stem)) {
            Move-Item -LiteralPath $f.FullName -Destination (Join-Path $limboSe $f.Name) -Force
            [void]$skippedProtected.Add($stem)
            continue
        }
        Move-Item -LiteralPath $f.FullName -Destination (Join-Path $limboSe $f.Name) -Force
    }
    foreach ($d in Get-ChildItem -LiteralPath $secDir -Directory | Sort-Object Name) {
        $uuid = $d.Name
        $limboEnt = Join-Path $limboSe $uuid
        $coreDest = Join-Path $secRoot $uuid
        $starterSec = Join-Path $starterBase "security\$uuid"
        Ensure-Dir (Join-Path $starterBase 'security')

        if (Test-Path -LiteralPath $coreDest) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            continue
        }

        $specJson = Join-Path $d.FullName 'spec.json'
        $route = Get-SecurityRouting $specJson

        if ($route -eq 'motor') {
            Move-Item -LiteralPath $d.FullName -Destination $coreDest -Force
            continue
        }

        if ($claimedSecUuid.ContainsKey($uuid)) {
            Ensure-Dir (Split-Path $limboEnt -Parent)
            Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
        } else {
            if (Test-Path -LiteralPath $starterSec) {
                Ensure-Dir (Split-Path $limboEnt -Parent)
                Move-Item -LiteralPath $d.FullName -Destination $limboEnt -Force
            } else {
                Move-Item -LiteralPath $d.FullName -Destination $starterSec -Force
                $claimedSecUuid[$uuid] = $true
            }
        }
    }
}

# --- PASO 4 evolution ---
Ensure-Dir (Join-Path $limboArchetype 'evolution')
foreach ($n in @(1, 2, 3, 4)) {
    $evo = Join-Path $RepoRoot "SddIA_$n\evolution"
    $dest = Join-Path $limboArchetype "evolution\SddIA_$n"
    if (Test-Path -LiteralPath $evo) {
        if (Test-Path -LiteralPath $dest) {
            throw "Destino evolution ya existe: $dest"
        }
        Move-Item -LiteralPath $evo -Destination $dest -Force
    }
}

# --- PASO 5 poda ---
function Remove-EmptyDirs([string]$root) {
    if (-not (Test-Path -LiteralPath $root)) { return }
    Get-ChildItem -LiteralPath $root -Directory -Recurse -Force |
        Sort-Object { $_.FullName.Length } -Descending |
        ForEach-Object {
            try {
                $has = Get-ChildItem -LiteralPath $_.FullName -Force -ErrorAction SilentlyContinue
                if ($null -eq $has -or $has.Count -eq 0) {
                    Remove-Item -LiteralPath $_.FullName -Force -Recurse -ErrorAction SilentlyContinue
                }
            } catch { }
        }
}
foreach ($n in @(1, 2, 3, 4)) {
    Remove-EmptyDirs (Join-Path $RepoRoot "SddIA_$n")
}

# --- PASO 6 ---
$evoRoot = Join-Path $limboArchetype 'evolution'
$timeline = 0
if (Test-Path -LiteralPath $evoRoot) {
    $timeline = (Get-ChildItem -LiteralPath $evoRoot -File -Recurse -Force | Measure-Object).Count
}

$out = [ordered]@{
    sequence              = 'definitive-epistemological-triage'
    core_norms_injected   = @($coreNormsInjected | Sort-Object)
    skipped_protected     = @($skippedProtected | Sort-Object)
    starter_kit_populated = @{
        principles_folders_moved = $principlesFoldersMoved
        patterns_folders_moved   = $patternsFoldersMoved
        norms_files_moved          = $normsFilesMoved
    }
    timeline_sealed       = $timeline
    integrity_check       = 'Ejecucion determinista finalizada.'
}
($out | ConvertTo-Json -Depth 6 -Compress) | Write-Output
