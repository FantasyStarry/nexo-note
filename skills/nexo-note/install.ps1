# Install the nexo-note skill into the local Trae skills directory.
$sourceDir = $PSScriptRoot
$targetDir = Join-Path (Join-Path (Join-Path $env:USERPROFILE ".trae-cn") "skills") "nexo-note"

Write-Host "Installing nexo-note skill to $targetDir ..."

if (-not (Test-Path $targetDir)) {
    New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
}

Copy-Item -Path (Join-Path $sourceDir "skill.yaml") -Destination $targetDir -Force
Copy-Item -Path (Join-Path $sourceDir "instructions.md") -Destination $targetDir -Force
Copy-Item -Path (Join-Path $sourceDir "examples.md") -Destination $targetDir -Force

Write-Host "nexo-note skill installed successfully."
