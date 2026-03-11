param()

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot

$demoMatrix = @(
  @{ name = 'ts-basic'; path = Join-Path $repoRoot 'demo\ts\basic'; build = 'npm run build' },
  @{ name = 'ts-variants'; path = Join-Path $repoRoot 'demo\ts\variants'; build = 'npm run build' },
  @{ name = 'ts-theme'; path = Join-Path $repoRoot 'demo\ts\theme'; build = 'npm run build' },
  @{ name = 'ts-workspace'; path = Join-Path $repoRoot 'demo\ts\workspace'; build = 'npm run build' },
  @{ name = 'react-basic'; path = Join-Path $repoRoot 'demo\react\basic'; build = 'npm run build' },
  @{ name = 'react-variants'; path = Join-Path $repoRoot 'demo\react\variants'; build = 'npm run build' },
  @{ name = 'react-theme'; path = Join-Path $repoRoot 'demo\react\theme'; build = 'npm run build' },
  @{ name = 'react-workspace'; path = Join-Path $repoRoot 'demo\react\workspace'; build = 'npm run build' },
  @{ name = 'vue-basic'; path = Join-Path $repoRoot 'demo\vue\basic'; build = 'npm run build' },
  @{ name = 'vue-variants'; path = Join-Path $repoRoot 'demo\vue\variants'; build = 'npm run build' },
  @{ name = 'vue-theme'; path = Join-Path $repoRoot 'demo\vue\theme'; build = 'npm run build' },
  @{ name = 'vue-workspace'; path = Join-Path $repoRoot 'demo\vue\workspace'; build = 'npm run build' }
)

foreach ($demo in $demoMatrix) {
  Push-Location $demo.path
  try {
    Write-Host "==> $($demo.name): generate motif.css"
    cargo run -p motif-core -- .
    if ($LASTEXITCODE -ne 0) { throw "motif generation failed for $($demo.name)" }

    Write-Host "==> $($demo.name): install dependencies"
    npm install --no-package-lock
    if ($LASTEXITCODE -ne 0) { throw "npm install failed for $($demo.name)" }

    Write-Host "==> $($demo.name): build"
    Invoke-Expression $demo.build
    if ($LASTEXITCODE -ne 0) { throw "build failed for $($demo.name)" }
  }
  finally {
    Pop-Location
  }
}
