$ErrorActionPreference = "Stop"

$ProjectName = "snow"
$RustInstaller = "https://www.rust-lang.org/tools/install"
$Mode = "release"

function CheckRustInstallation {
    try {
        $null = rustc --version
        Write-Host "We have verified that Rust is installed in your machine." -ForegroundColor Green
        return $true
    } catch {
        Write-Host "Rust is NOT installed." -ForegroundColor Red
        Write-Host "Please download and install Rust from: $RustInstaller" -ForegroundColor Yellow
        exit 1
    }
}

function CompileProject {
    Write-Host "Compiling project in Release (Optimized) mode..." -ForegroundColor Cyan

    $Command = "cargo build --release"
    
    Invoke-Expression $Command
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Compilation failed!" -ForegroundColor Red
        exit 1
    }
    Write-Host "Compilation successful." -ForegroundColor Green
}

function GetExecutionPath {
    return "target\release\$ProjectName.exe"
}

CheckRustInstallation

$ExecutablePath = GetExecutionPath

if (-not (Test-Path $ExecutablePath)) {
    Write-Host "Executable not found at '$ExecutablePath'." -ForegroundColor Yellow
    
    $InputCompile = Read-Host "Would you like to compile the server in Release mode now? (y/N). Enter to default (N): "
    
    if ($InputCompile -eq "y" -or $InputCompile -eq "Y") {
        CompileProject
    } else {
        Write-Host "Aborting. Please compile the server first." -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "Executable found. Skipping compilation." -ForegroundColor Green
}

Write-Host "Starting Snow Server..." -ForegroundColor Yellow
& $ExecutablePath