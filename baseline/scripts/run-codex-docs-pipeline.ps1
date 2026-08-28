param(
    [Parameter(Mandatory = $true)]
    [string]$RootDir,

    [Parameter(Mandatory = $true)]
    [string]$PromptFile,

    [string]$CodexBin = "codex",

    [string]$SessionLogTemplate = ""
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = "Stop"

function Resolve-ExistingFilePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$BaseDir
    )

    $candidatePath = $Path
    if (-not [System.IO.Path]::IsPathRooted($candidatePath)) {
        $candidatePath = Join-Path -Path $BaseDir -ChildPath $candidatePath
    }

    if (-not (Test-Path -LiteralPath $candidatePath -PathType Leaf)) {
        throw "File not found: $candidatePath"
    }

    return (Resolve-Path -LiteralPath $candidatePath).Path
}

function Resolve-CommandPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$CommandName
    )

    if ([System.IO.Path]::IsPathRooted($CommandName) -or $CommandName.IndexOf("\") -ge 0 -or $CommandName.IndexOf("/") -ge 0) {
        if (-not (Test-Path -LiteralPath $CommandName -PathType Leaf)) {
            throw "Codex executable not found: $CommandName"
        }

        return (Resolve-Path -LiteralPath $CommandName).Path
    }

    $command = Get-Command -Name $CommandName -CommandType Application -ErrorAction Stop |
        Sort-Object -Property @{
            Expression = {
                if ($_.Source -match "\\openai\.chatgpt-" -and $_.Source -like "*.exe") { return 0 }
                if ($_.Source -like "*.exe") { return 1 }
                if ($_.Source -like "*.cmd") { return 2 }
                return 3
            }
        } |
        Select-Object -First 1

    return $command.Source
}

function Test-WritableDirectory {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    try {
        New-Item -ItemType Directory -Force -Path $Path | Out-Null

        $probeFile = Join-Path -Path $Path -ChildPath ("write-test-{0}.tmp" -f ([System.Guid]::NewGuid().ToString("N")))
        [System.IO.File]::WriteAllText($probeFile, "ok")
        Remove-Item -LiteralPath $probeFile -Force -ErrorAction SilentlyContinue

        return $true
    }
    catch {
        return $false
    }
}

function Resolve-CodexHomePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RootPath
    )

    if (-not [string]::IsNullOrWhiteSpace($env:CODEX_HOME)) {
        return $env:CODEX_HOME
    }

    $defaultCodexHome = Join-Path -Path $HOME -ChildPath ".codex"
    if (Test-WritableDirectory -Path $defaultCodexHome) {
        return $defaultCodexHome
    }

    return (Join-Path -Path $RootPath -ChildPath ".codex-home")
}

function Resolve-SessionLogTemplatePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RootPath,

        [string]$TemplatePath
    )

    if (-not [string]::IsNullOrWhiteSpace($TemplatePath)) {
        return (Resolve-ExistingFilePath -Path $TemplatePath -BaseDir $RootPath)
    }

    $latestTemplate = Get-ChildItem -LiteralPath $RootPath -File -Filter "log-*.md" -ErrorAction SilentlyContinue |
        Sort-Object -Property LastWriteTimeUtc -Descending |
        Select-Object -First 1

    if ($null -eq $latestTemplate) {
        return $null
    }

    return $latestTemplate.FullName
}

function New-ModeledSessionPromptSequence {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ProjectBrief
    )

    return @(
        [pscustomobject]@{ Number = "01"; Text = $ProjectBrief },
        [pscustomobject]@{ Number = "02"; Text = "PLEASE IMPLEMENT THIS PLAN:`nRequirements Document" },
        [pscustomobject]@{ Number = "03"; Text = "From this requirements document generate a SWOT analysis for the project and put it in SWOT.md." },
        [pscustomobject]@{ Number = "04"; Text = "PLEASE IMPLEMENT THIS PLAN:`nSWOT Analysis Document" },
        [pscustomobject]@{ Number = "05"; Text = "Now we have the SWOT and requirements document, generate a mission and vision for the project. This should guide the visual identity of the project." },
        [pscustomobject]@{ Number = "06"; Text = "PLEASE IMPLEMENT THIS PLAN:`nMission And Vision Brand Foundation Document" },
        [pscustomobject]@{ Number = "07"; Text = "From these docs, generate a design language and visual identity document named VisualIdentity.md." },
        [pscustomobject]@{ Number = "08"; Text = "PLEASE IMPLEMENT THIS PLAN:`nVisual Identity And Design Language Document" },
        [pscustomobject]@{ Number = "09"; Text = "From all the documents, especially the visual identity one, generate a prompt package that can create a color palette and logo for the project. Also suggest names and record the selected working name for now." },
        [pscustomobject]@{ Number = "10"; Text = "PLEASE IMPLEMENT THIS PLAN:`nNaming And Brand Prompt Package" },
        [pscustomobject]@{ Number = "11"; Text = "Now the next step is making a technical architecture document from all documents, especially the requirements document. Aim for strong performance, maintainability, and scalable architecture." },
        [pscustomobject]@{ Number = "12"; Text = "PLEASE IMPLEMENT THIS PLAN:`nTechnical Architecture Document" },
        [pscustomobject]@{ Number = "13"; Text = "Create topic-based feature research and put the results in Research-01-topic.md style files." },
        [pscustomobject]@{ Number = "14"; Text = "Make a log for all the prompts used here, include dates, and put the log in Docs using the log-date-time.md naming pattern." }
    )
}

function Convert-PromptSequenceToInstructionText {
    param(
        [Parameter(Mandatory = $true)]
        [object[]]$PromptSequence
    )

    return (($PromptSequence | ForEach-Object {
        "- {0}: {1}" -f $_.Number, ($_.Text -replace "\r?\n", " ")
    }) -join "`r`n")
}

function New-RequiredArtifactsText {
    param(
        [Parameter(Mandatory = $true)]
        [string]$GeneratedLogFileName
    )

    $artifactLines = @(
        "- Docs\README.md: documentation index, sequencing guide, and contributor starting point."
        "- Docs\requirements.md: product and technical requirements grounded in the project brief."
        "- Docs\SWOT.md: SWOT analysis based on the requirements and competitive framing of the project."
        "- Docs\MissionVision.md: mission, vision, and brand pillars derived from the requirements and SWOT."
        "- Docs\VisualIdentity.md: design language and visual identity guidance grounded in the prior docs."
        "- Docs\BrandPrompt.md: naming, logo, and palette prompt package informed by the visual identity."
        "- Docs\TechnicalArchitecture.md: technical architecture focused on performance, maintainability, and scalability."
        "- Docs\Research-01-<topic>.md through Docs\Research-05-<topic>.md: at least five topic-based research files on the most important project-specific areas."
        "- Docs\${GeneratedLogFileName}: a prompt/session log modeled after the supplied session log template, using the current run timestamp."
    )

    return ($artifactLines -join "`r`n")
}

$resolvedRootDir = (Resolve-Path -LiteralPath $RootDir).Path
$resolvedPromptFile = Resolve-ExistingFilePath -Path $PromptFile -BaseDir $resolvedRootDir
$resolvedCodexBin = Resolve-CommandPath -CommandName $CodexBin
$docsDir = Join-Path -Path $resolvedRootDir -ChildPath "Docs"
$codexHome = Resolve-CodexHomePath -RootPath $resolvedRootDir
$summaryFile = Join-Path -Path $docsDir -ChildPath "_pipeline-last-message.txt"
$codexTempDir = Join-Path -Path $codexHome -ChildPath ".tmp"

New-Item -ItemType Directory -Force -Path $docsDir | Out-Null
New-Item -ItemType Directory -Force -Path $codexHome | Out-Null
New-Item -ItemType Directory -Force -Path $codexTempDir | Out-Null

$env:CODEX_HOME = $codexHome

$projectPrompt = [System.IO.File]::ReadAllText($resolvedPromptFile).Trim()
if ([string]::IsNullOrWhiteSpace($projectPrompt)) {
    throw "Prompt file is empty: $resolvedPromptFile"
}

$sessionLogTemplatePath = Resolve-SessionLogTemplatePath -RootPath $resolvedRootDir -TemplatePath $SessionLogTemplate
$sessionLogTemplateContent = ""
if ($sessionLogTemplatePath) {
    $sessionLogTemplateContent = [System.IO.File]::ReadAllText($sessionLogTemplatePath).Trim()
}

$runTimestamp = Get-Date
$runTimestampText = $runTimestamp.ToString("yyyy-MM-dd HH:mm:ss")
$runDateText = $runTimestamp.ToString("yyyy-MM-dd")
$generatedLogFileName = "log-{0}.md" -f $runTimestamp.ToString("yyyy-MM-dd-HH-mm-ss")
$modeledPromptSequence = New-ModeledSessionPromptSequence -ProjectBrief $projectPrompt
$modeledPromptSequenceText = Convert-PromptSequenceToInstructionText -PromptSequence $modeledPromptSequence
$requiredArtifactsText = New-RequiredArtifactsText -GeneratedLogFileName $generatedLogFileName

$instructionSections = @(
    "You are working in the project root at `"$resolvedRootDir`".",
    "",
    "Execute a staged documentation pipeline that mirrors the workflow, artifact progression, and session logging style shown in the reference session log template.",
    "Do not copy the pomodoro-specific content from the template. Use it only as a structural and process reference, and adapt all output to the current project brief.",
    "",
    "Project brief:",
    $projectPrompt,
    "",
    "Current run metadata:",
    "- Session date: $runDateText",
    "- Session timestamp: $runTimestampText",
    "- Generated log file name: Docs\$generatedLogFileName",
    "",
    "Modeled session prompt sequence:",
    $modeledPromptSequenceText,
    "",
    "Required artifacts:",
    $requiredArtifactsText,
    "",
    "Pipeline expectations:",
    "- The docs should feel like the result of a staged session, not one disconnected bulk dump.",
    "- Each later document should clearly build on the earlier ones.",
    "- Record concrete planning decisions that materially shaped the docs, similar to the template log.",
    "- Choose a sensible working project name if the brief does not include one, and record it in the brand prompt package and the generated session log.",
    "- For the research stage, create at least five topic files using the Research-01-<topic>.md pattern with project-relevant slugs.",
    "- The technical architecture doc should explicitly reference the requirements document and optimize for performance, maintainability, and scalability where relevant.",
    "- Keep Markdown concise, practical, and ASCII-only unless an existing file requires otherwise.",
    "- Update files directly in the Docs directory.",
    "- End with a short summary of what you created or updated.",
    "",
    "Generated session log requirements:",
    "- Follow the same major sections as the reference log: Notes, User Prompt Log, Planning Decisions Captured During The Session, Resulting Session Artifacts, and Summary.",
    "- Use the modeled session prompt sequence above as the basis of the prompt log entries.",
    "- Date-stamp entries using the current run date and current run timestamp above.",
    "- List the resulting artifacts created in this run with their Docs paths.",
    "",
    "Reference session log template path:",
    $(if ($sessionLogTemplatePath) { $sessionLogTemplatePath } else { "(none found)" }),
    "",
    "Reference session log template contents:",
    $(if ([string]::IsNullOrWhiteSpace($sessionLogTemplateContent)) { "No session log template file was found in the project root, so rely on the modeled prompt sequence above." } else { $sessionLogTemplateContent })
)

$instruction = ($instructionSections -join "`r`n").Trim()

$codexArgs = @(
    "-a", "never",
    "exec",
    "--sandbox", "workspace-write",
    "--skip-git-repo-check",
    "--ephemeral",
    "--color", "never",
    "--cd", $resolvedRootDir,
    "--output-last-message", $summaryFile,
    "-"
)

Write-Host "Using prompt file: $resolvedPromptFile"
Write-Host "Writing documentation to: $docsDir"
Write-Host "Using CODEX_HOME: $codexHome"
Write-Host "Running Codex with: $resolvedCodexBin"
if ($sessionLogTemplatePath) {
    Write-Host "Using session log template: $sessionLogTemplatePath"
}
Write-Host "Generated log target: $generatedLogFileName"
Write-Host ""

$instruction | & $resolvedCodexBin @codexArgs
$exitCode = $LASTEXITCODE

if ($exitCode -ne 0) {
    exit $exitCode
}

if (Test-Path -LiteralPath $summaryFile) {
    Write-Host ""
    Write-Host "Pipeline summary saved to: $summaryFile"
}
