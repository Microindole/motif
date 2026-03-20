param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
  node scripts/node/quality-fast.mjs
}
finally {
  Pop-Location
}
