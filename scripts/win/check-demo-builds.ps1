param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
  cargo run -p xtask -- demo-builds
}
finally {
  Pop-Location
}
