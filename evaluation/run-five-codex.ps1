[CmdletBinding()]
param(
    [string]$DataRoot = (Join-Path $PSScriptRoot "improved"),
    [string]$LogRoot = (Join-Path $PSScriptRoot "logs"),
    [ValidateSet("debug", "release")]
    [string]$BuildProfile = "debug"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $false

$repositoryRoot = Split-Path -Parent $PSScriptRoot
$runs = @(
    @{ Slug = "last-light-courier"; Name = "Last Light Courier" },
    @{ Slug = "pigeon-payroll"; Name = "Pigeon Payroll" },
    @{ Slug = "borrowed-orbit"; Name = "Borrowed Orbit" },
    @{ Slug = "dead-air-dispatch"; Name = "Dead Air Dispatch" },
    @{ Slug = "half-mech-heroes"; Name = "Half-Mech Heroes" }
)
$failures = [System.Collections.Generic.List[string]]::new()

Push-Location $repositoryRoot
try {
    $cargoArguments = @("build")
    if ($BuildProfile -eq "release") {
        $cargoArguments += "--release"
    }

    & cargo @cargoArguments
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed with exit code $LASTEXITCODE."
    }

    $pipeline = Join-Path $repositoryRoot "target\$BuildProfile\project-init.exe"
    if (-not (Test-Path -LiteralPath $pipeline -PathType Leaf)) {
        throw "Built pipeline executable was not found at $pipeline."
    }

    New-Item -ItemType Directory -Path $DataRoot -Force | Out-Null
    New-Item -ItemType Directory -Path $LogRoot -Force | Out-Null

    foreach ($run in $runs) {
        $brief = Join-Path $PSScriptRoot "cases\$($run.Slug).txt"
        $dataDirectory = Join-Path $DataRoot $run.Slug
        $log = Join-Path $LogRoot "$($run.Slug).log"

        if (-not (Test-Path -LiteralPath $brief -PathType Leaf)) {
            $failures.Add("$($run.Name): brief not found at $brief")
            Write-Warning $failures[$failures.Count - 1]
            continue
        }

        Write-Host "Running $($run.Name)..."

        & $pipeline `
            --data-dir $dataDirectory `
            run `
            --brief $brief `
            --name $run.Name `
            --auto-answer `
            --approval autonomous 2>&1 |
            Tee-Object -FilePath $log

        $exitCode = $LASTEXITCODE
        if ($exitCode -ne 0) {
            $failures.Add("$($run.Name): pipeline exited with code $exitCode")
        }
    }
}
finally {
    Pop-Location
}

if ($failures.Count -gt 0) {
    throw "One or more Codex pipeline runs failed:`n - $($failures -join "`n - ")"
}

Write-Host "All five Codex pipeline runs completed."
