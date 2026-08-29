$ErrorActionPreference = "Stop"

function Invoke-CargoCommand {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE."
    }
}

$repositoryRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..\..\..")).Path
$manifestPath = Join-Path $repositoryRoot "Cargo.toml"

if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
    throw "Cargo.toml was not found at the expected repository root: $repositoryRoot"
}

Push-Location $repositoryRoot
try {
    Invoke-CargoCommand @("fmt", "--check")
    Invoke-CargoCommand @("clippy", "--locked", "--all-targets", "--all-features", "--", "-D", "warnings")
    Invoke-CargoCommand @("test", "--locked", "--all-targets", "--all-features")
    Invoke-CargoCommand @("build", "--release", "--locked")

    $metadataJson = & cargo metadata --locked --no-deps --format-version 1
    if ($LASTEXITCODE -ne 0) {
        throw "cargo metadata failed with exit code $LASTEXITCODE."
    }

    $metadata = $metadataJson | ConvertFrom-Json
    $package = $metadata.packages | Where-Object { $_.manifest_path -eq $manifestPath.Replace("\", "/") } | Select-Object -First 1
    if ($null -eq $package) {
        $package = $metadata.packages | Select-Object -First 1
    }
    if ($null -eq $package) {
        throw "Cargo metadata did not return a package."
    }

    $hostLine = & rustc -Vv | Select-String -Pattern "^host: " | Select-Object -First 1
    if ($LASTEXITCODE -ne 0 -or $null -eq $hostLine) {
        throw "Unable to determine the local Rust target triple."
    }
    $targetTriple = $hostLine.Line.Substring("host: ".Length).Trim()

    $binaryFileName = $package.name
    if ($env:OS -eq "Windows_NT") {
        $binaryFileName += ".exe"
    }

    $sourceBinary = Join-Path $repositoryRoot (Join-Path "target\release" $binaryFileName)
    if (-not (Test-Path -LiteralPath $sourceBinary -PathType Leaf)) {
        throw "The release binary was not found: $sourceBinary"
    }

    $artifactDirectoryName = "$($package.name)-v$($package.version)-$targetTriple"
    $artifactDirectory = Join-Path (Join-Path $repositoryRoot "builds") $artifactDirectoryName
    New-Item -ItemType Directory -Path $artifactDirectory -Force | Out-Null

    $artifactBinary = Join-Path $artifactDirectory $binaryFileName
    Copy-Item -LiteralPath $sourceBinary -Destination $artifactBinary -Force

    $hash = (Get-FileHash -LiteralPath $artifactBinary -Algorithm SHA256).Hash.ToLowerInvariant()
    $checksumPath = "$artifactBinary.sha256"
    Set-Content -LiteralPath $checksumPath -Value "$hash  $binaryFileName" -Encoding utf8NoBOM

    Write-Output "Build artifact: $artifactBinary"
    Write-Output "SHA-256: $hash"
    Write-Output "Checksum file: $checksumPath"
}
finally {
    Pop-Location
}
