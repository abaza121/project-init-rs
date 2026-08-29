[CmdletBinding()]
param(
    [string]$BriefRoot,
    [string]$DataRoot,
    [string]$LogRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($BriefRoot)) {
    $BriefRoot = Join-Path $PSScriptRoot "cases"
}
if ([string]::IsNullOrWhiteSpace($DataRoot)) {
    $DataRoot = Join-Path $PSScriptRoot "improved"
}
if ([string]::IsNullOrWhiteSpace($LogRoot)) {
    $LogRoot = Join-Path $PSScriptRoot "logs"
}

$pipeline = Join-Path $PSScriptRoot "project-init.exe"
if (-not (Test-Path -LiteralPath $pipeline -PathType Leaf)) {
    throw "project-init.exe must be in the same directory as this script: $PSScriptRoot"
}

$runs = @(
    @{ Slug = "last-light-courier"; Name = "Last Light Courier" },
    @{ Slug = "pigeon-payroll"; Name = "Pigeon Payroll" },
    @{ Slug = "borrowed-orbit"; Name = "Borrowed Orbit" },
    @{ Slug = "dead-air-dispatch"; Name = "Dead Air Dispatch" },
    @{ Slug = "half-mech-heroes"; Name = "Half-Mech Heroes" }
)
$failures = @()

New-Item -ItemType Directory -Path $DataRoot -Force | Out-Null
New-Item -ItemType Directory -Path $LogRoot -Force | Out-Null

foreach ($run in $runs) {
    $brief = Join-Path $BriefRoot "$($run.Slug).txt"
    $dataDirectory = Join-Path $DataRoot $run.Slug
    $log = Join-Path $LogRoot "$($run.Slug).log"

    if (-not (Test-Path -LiteralPath $brief -PathType Leaf)) {
        $message = "$($run.Name): brief not found at $brief"
        $failures += $message
        Write-Warning $message
        continue
    }

    Write-Host "Running $($run.Name)..."

    & $pipeline `
        --data-dir $dataDirectory `
        run `
        --brief $brief `
        --name $run.Name `
        --auto-answer `
        --approval autonomous |
        Tee-Object -FilePath $log

    $exitCode = $LASTEXITCODE
    if ($exitCode -ne 0) {
        $failures += "$($run.Name): pipeline exited with code $exitCode"
    }
}

if ($failures.Count -gt 0) {
    throw "One or more Codex pipeline runs failed:`n - $($failures -join "`n - ")"
}

Write-Host "All five Codex pipeline runs completed."
