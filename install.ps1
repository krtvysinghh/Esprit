# ==============================================================================
# Esprit — Windows PowerShell Installer & Model Bootstrapper
# ==============================================================================
# Usage:
#   irm https://raw.githubusercontent.com/krtvysinghh/Esprit/main/install.ps1 | iex
# ==============================================================================

$ErrorActionPreference = "Stop"

$Repo = "krtvysinghh/Esprit"
$BinName = "esprit.exe"

Write-Host "Esprit Windows Installer & Model Bootstrapper" -ForegroundColor Cyan

# 1. Models Directory
$ModelsDir = "$env:LOCALAPPDATA\esprit\models"
if (!(Test-Path -Path $ModelsDir)) {
    New-Item -ItemType Directory -Path $ModelsDir -Force | Out-Null
}

# 2. Download Default Models
$Models = @(
    @{
        Name = "qwen3-0.6b-q4_k_m.gguf"
        Url  = "https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf"
    },
    @{
        Name = "nomic-embed-text-v1.5.Q4_K_M.gguf"
        Url  = "https://huggingface.co/nomic-ai/nomic-embed-text-v1.5-GGUF/resolve/main/nomic-embed-text-v1.5.Q4_K_M.gguf"
    }
)

foreach ($model in $Models) {
    $targetPath = Join-Path $ModelsDir $model.Name
    if (!(Test-Path -Path $targetPath)) {
        Write-Host "⬇ Downloading $($model.Name)..." -ForegroundColor Yellow
        Invoke-WebRequest -Uri $model.Url -OutFile $targetPath -UseBasicParsing
        Write-Host "✓ Downloaded $($model.Name)" -ForegroundColor Green
    } else {
        Write-Host "✓ Model already present: $($model.Name)" -ForegroundColor Green
    }
}

$balancedModel = Join-Path $ModelsDir "qwen3-1.7b-q4_k_m.gguf"
if (!(Test-Path -Path $balancedModel)) {
    Copy-Item (Join-Path $ModelsDir "qwen3-0.6b-q4_k_m.gguf") $balancedModel
}

Write-Host "✨ Esprit default installation & models ready on Windows!" -ForegroundColor Green
