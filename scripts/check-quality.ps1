param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $repoRoot

$failures = New-Object System.Collections.Generic.List[string]
$warnings = New-Object System.Collections.Generic.List[string]

function Add-Failure([string]$message) {
  $script:failures.Add($message)
  Write-Host "FAIL: $message" -ForegroundColor Red
}

function Add-Warning([string]$message) {
  $script:warnings.Add($message)
  Write-Host "WARN: $message" -ForegroundColor Yellow
}

function Normalize-RepoPath([string]$path) {
  return $path.Replace('\\', '/')
}

function Invoke-CommandStep([string]$name, [scriptblock]$action) {
  Write-Host "==> $name"
  & $action
  if ($LASTEXITCODE -ne 0) {
    Add-Failure("$name failed with exit code $LASTEXITCODE")
  }
}

function Get-TrackedFiles() {
  $files = git ls-files
  if ($LASTEXITCODE -ne 0) {
    throw 'git ls-files failed'
  }
  return $files | Where-Object { $_ -and $_.Trim().Length -gt 0 }
}

function Test-FileLineLimits([string[]]$trackedFiles) {
  foreach ($file in $trackedFiles) {
    $normalized = Normalize-RepoPath $file
    if (-not (Test-Path $file) -or (Get-Item $file).PSIsContainer) {
      continue
    }

    $limit = $null
    switch -Regex ($normalized) {
      '^core/src/.+\.rs$' { $limit = 320; break }
      '^core/tests/.+\.rs$' { $limit = 420; break }
      '^scripts/.+\.ps1$' { $limit = 320; break }
      '^agent/.+\.md$' { $limit = 400; break }
      '^tokens/.+\.json$' { $limit = 220; break }
      '^(demo|cases)/.+\.(html|ts|tsx|vue|md|json)$' { $limit = 260; break }
      '^\.github/.+\.(yml|yaml|md)$' { $limit = 260; break }
    }

    if ($null -eq $limit) {
      continue
    }

    $lines = (Get-Content $file | Measure-Object -Line).Lines
    if ($lines -gt $limit) {
      Add-Failure("$normalized is $lines lines, exceeds limit $limit")
    } elseif ($normalized -like 'core/src/*' -and $lines -gt 240) {
      Add-Warning("$normalized is already large at $lines lines")
    }
  }
}

function Test-DirectoryFlatness([string[]]$trackedFiles) {
  $grouped = $trackedFiles |
    Where-Object { $_ -notmatch '(^|/)(target|node_modules|dist|coverage|\.vite)(/|$)' } |
    Group-Object { Normalize-RepoPath (Split-Path $_ -Parent) }

  foreach ($group in $grouped) {
    if ([string]::IsNullOrWhiteSpace($group.Name)) {
      continue
    }
    if ($group.Count -gt 12) {
      Add-Failure("directory $($group.Name) has $($group.Count) tracked files; limit is 12")
    }
  }
}

function Test-GeneratedArtifactPollution([string[]]$trackedFiles) {
  $forbidden = $trackedFiles | Where-Object {
    $_ -match '(^|/)(target|node_modules|dist|coverage|\.vite)(/|$)' -or
    $_ -match '(^|/)motif\.css$' -or
    $_ -match '\.tsbuildinfo$'
  }

  foreach ($file in $forbidden) {
    Add-Failure("tracked generated artifact detected: $(Normalize-RepoPath $file)")
  }
}

function Test-ForbiddenPatterns([string[]]$trackedFiles) {
  $libraryFiles = $trackedFiles | Where-Object { $_ -match '^core/src/.+\.rs$' -and $_ -notmatch '^core/src/main\.rs$' }

  $patterns = @(
    @{ Regex = '\bunsafe\b'; Message = 'unsafe is forbidden outside explicit review' },
    @{ Regex = 'transmute'; Message = 'transmute is forbidden' },
    @{ Regex = 'MaybeUninit'; Message = 'MaybeUninit is forbidden' },
    @{ Regex = 'todo!\s*\('; Message = 'todo! is forbidden' },
    @{ Regex = 'unimplemented!\s*\('; Message = 'unimplemented! is forbidden' },
    @{ Regex = 'dbg!\s*\('; Message = 'dbg! is forbidden' },
    @{ Regex = '\.unwrap\s*\('; Message = 'unwrap() is forbidden in library code' },
    @{ Regex = '\.expect\s*\('; Message = 'expect() is forbidden in library code' },
    @{ Regex = '\bprintln!\s*\('; Message = 'println! is forbidden in library code' },
    @{ Regex = '\beprintln!\s*\('; Message = 'eprintln! is forbidden in library code' }
  )

  foreach ($file in $libraryFiles) {
    $content = Get-Content $file -Raw
    foreach ($pattern in $patterns) {
      if ($content -match $pattern.Regex) {
        Add-Failure("$(Normalize-RepoPath $file): $($pattern.Message)")
      }
    }
  }
}

function Test-ContextEntryDocs() {
  $requiredDocs = @(
    'agent/product.md',
    'agent/quality.md',
    'agent/presets.md',
    'agent/scope.md',
    'agent/architecture.md',
    'agent/status.md',
    'agent/rules.md'
  )

  $context = Get-Content agent/context.md -Raw
  foreach ($doc in $requiredDocs) {
    if (-not (Test-Path $doc)) {
      Add-Failure("required doc missing: $doc")
      continue
    }
    if ($context -notmatch [regex]::Escape($doc)) {
      Add-Failure("agent/context.md is missing entry for $doc")
    }
  }
}

function Test-NamingPatterns([string[]]$trackedFiles) {
  $suspiciousTokens = @('helper', 'util', 'utils', 'manager', 'service', 'handler', 'processor', 'temp', 'misc', 'final', 'old', 'new')
  $grouped = $trackedFiles | Group-Object { Normalize-RepoPath (Split-Path $_ -Parent) }

  foreach ($group in $grouped) {
    $prefixCounts = @{}
    $suffixCounts = @{}

    foreach ($file in $group.Group) {
      $base = [System.IO.Path]::GetFileNameWithoutExtension($file)
      $parts = $base -split '[_-]'
      if ($parts.Count -ge 2) {
        $prefix = $parts[0]
        $suffix = $parts[$parts.Count - 1]
        if (-not $prefixCounts.ContainsKey($prefix)) { $prefixCounts[$prefix] = 0 }
        if (-not $suffixCounts.ContainsKey($suffix)) { $suffixCounts[$suffix] = 0 }
        $prefixCounts[$prefix] += 1
        $suffixCounts[$suffix] += 1
      }
    }

    foreach ($token in $suspiciousTokens) {
      if ($suffixCounts.ContainsKey($token) -and $suffixCounts[$token] -ge 4) {
        Add-Warning("directory $($group.Name) has $($suffixCounts[$token]) files ending with suspicious token '$token'")
      }
      if ($prefixCounts.ContainsKey($token) -and $prefixCounts[$token] -ge 4) {
        Add-Warning("directory $($group.Name) has $($prefixCounts[$token]) files starting with suspicious token '$token'")
      }
    }
  }
}

function Test-CommentHeuristics([string[]]$trackedFiles) {
  $sourceFiles = $trackedFiles | Where-Object { $_ -match '\.(rs|ts|tsx|vue|ps1)$' }

  foreach ($file in $sourceFiles) {
    $lines = Get-Content $file
    if ($lines.Count -lt 25) {
      continue
    }

    $commentLines = ($lines | Where-Object {
      $_ -match '^\s*//' -or $_ -match '^\s*/\*' -or $_ -match '^\s*\*' -or $_ -match '^\s*<!--' -or $_ -match '^\s*#'
    }).Count
    $commentRatio = if ($lines.Count -gt 0) { $commentLines / $lines.Count } else { 0 }
    $normalized = Normalize-RepoPath $file

    if ($commentRatio -gt 0.28 -and $lines.Count -lt 140) {
      Add-Warning("$normalized has unusually high comment density ($([math]::Round($commentRatio * 100, 1))%)")
    }

    $branchSignals = ($lines | Where-Object { $_ -match '\b(if|match|for|while|switch)\b' }).Count
    if ($lines.Count -ge 80 -and $branchSignals -ge 8 -and $commentLines -eq 0) {
      Add-Warning("$normalized is complex and has no comments explaining constraints or tradeoffs")
    }
  }
}

$trackedFiles = Get-TrackedFiles

Test-GeneratedArtifactPollution -trackedFiles $trackedFiles
Test-FileLineLimits -trackedFiles $trackedFiles
Test-DirectoryFlatness -trackedFiles $trackedFiles
Test-ForbiddenPatterns -trackedFiles $trackedFiles
Test-ContextEntryDocs
Test-NamingPatterns -trackedFiles $trackedFiles
Test-CommentHeuristics -trackedFiles $trackedFiles

Invoke-CommandStep 'cargo fmt --all --check' { cargo fmt --all --check }
Invoke-CommandStep 'cargo clippy --workspace --all-targets --all-features -- -D warnings' { cargo clippy --workspace --all-targets --all-features -- -D warnings }
Invoke-CommandStep 'cargo test -p motif-core' { cargo test -p motif-core }
Invoke-CommandStep 'scripts/check-demo-builds.ps1' { & "$repoRoot\scripts\check-demo-builds.ps1" }

if ($warnings.Count -gt 0) {
  Write-Host ''
  Write-Host 'Soft warnings:' -ForegroundColor Yellow
  $warnings | ForEach-Object { Write-Host "- $_" -ForegroundColor Yellow }
}

if ($failures.Count -gt 0) {
  Write-Host ''
  Write-Host 'Hard gate failures:' -ForegroundColor Red
  $failures | ForEach-Object { Write-Host "- $_" -ForegroundColor Red }
  exit 1
}

Write-Host ''
Write-Host 'quality checks passed' -ForegroundColor Green
