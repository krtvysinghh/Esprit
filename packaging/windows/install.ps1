$ErrorActionPreference = "Stop"

$Repo = "krtvysinghh/Esprit"
$InstallDir = "$env:LOCALAPPDATA\Esprit"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Asset = $Release.assets | Where-Object { $_.name -match "windows-x86_64\.zip$" } | Select-Object -First 1

if (-not $Asset) {
    throw "Windows release asset not found."
}

$Zip = "$env:TEMP\esprit.zip"
Invoke-WebRequest $Asset.browser_download_url -OutFile $Zip

Expand-Archive $Zip -DestinationPath $InstallDir -Force
Remove-Item $Zip -Force

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$UserPath;$InstallDir",
        "User"
    )
}

Write-Host "Esprit installed successfully."
Write-Host "Restart your terminal, then run: esprit --help"
