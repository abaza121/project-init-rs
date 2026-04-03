@echo off
setlocal EnableExtensions DisableDelayedExpansion

set "SCRIPT_DIR=%~dp0"
if "%SCRIPT_DIR:~-1%"=="\" set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"
set "PIPELINE_SCRIPT=%SCRIPT_DIR%\scripts\run-codex-docs-pipeline.ps1"
set "POWERSHELL_EXE=%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe"
if not exist "%POWERSHELL_EXE%" set "POWERSHELL_EXE=powershell"

cd /d "%SCRIPT_DIR%"

set "PROMPT_FILE="
set "PROJECT_PROMPT="
set "RUNTIME_PROMPT_FILE="

if /i "%~1"=="-f" (
    if "%~2"=="" (
        echo Missing prompt file path after -f.
        echo Usage:
        echo   %~nx0 "Describe the project you want documented"
        echo   %~nx0 -f path\to\project-prompt.txt
        exit /b 1
    )
    set "PROMPT_FILE=%~2"
) else (
    set "PROJECT_PROMPT=%*"
)

if not defined PROJECT_PROMPT if not defined PROMPT_FILE (
    set /p "PROJECT_PROMPT=Enter the project prompt: "
)

if not defined PROJECT_PROMPT if not defined PROMPT_FILE (
    echo No project prompt provided.
    exit /b 1
)

if defined PROMPT_FILE if not exist "%PROMPT_FILE%" (
    echo Prompt file not found: "%PROMPT_FILE%"
    exit /b 1
)

if not exist "%PIPELINE_SCRIPT%" (
    echo Pipeline script not found: "%PIPELINE_SCRIPT%"
    exit /b 1
)

if not exist "%SCRIPT_DIR%\Docs" mkdir "%SCRIPT_DIR%\Docs"

if not defined CODEX_BIN set "CODEX_BIN=codex"

if defined PROJECT_PROMPT (
    set "RUNTIME_PROMPT_FILE=%TEMP%\codex-project-prompt-%RANDOM%%RANDOM%.txt"
    "%POWERSHELL_EXE%" -NoProfile -Command "Set-Content -LiteralPath $env:RUNTIME_PROMPT_FILE -Value $env:PROJECT_PROMPT -NoNewline -Encoding UTF8"
    if errorlevel 1 (
        echo Failed to create the runtime prompt file.
        exit /b 1
    )
    set "PROMPT_FILE=%RUNTIME_PROMPT_FILE%"
)

echo Running the Codex documentation pipeline...
echo.

"%POWERSHELL_EXE%" -NoProfile -ExecutionPolicy Bypass -File "%PIPELINE_SCRIPT%" -RootDir "%SCRIPT_DIR%" -PromptFile "%PROMPT_FILE%" -CodexBin "%CODEX_BIN%"
set "EXIT_CODE=%ERRORLEVEL%"

if defined RUNTIME_PROMPT_FILE if exist "%RUNTIME_PROMPT_FILE%" del "%RUNTIME_PROMPT_FILE%"

if not "%EXIT_CODE%"=="0" (
    echo.
    echo The Codex pipeline failed with exit code %EXIT_CODE%.
)

exit /b %EXIT_CODE%
