param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
  cargo run -p xtask -- quality
}
finally {
  Pop-Location
}
