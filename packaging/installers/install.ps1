$ErrorActionPreference = "Stop"

$Repo = "krtvysinghh/Esprit"
$InstallDir = "$env:LOCALAPPDATA\Esprit"

$Release = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Version = $Release.tag_name.TrimStart("v")

$Asset = $Release.assets |
    Where-Object { $_.name -eq "esprit-$Version-windows-x86_64.zip" } |
    Select-Object -First 1

if (-not $Asset) {
    throw "Windows release not found."
}

$Temp = Join-Path $env:TEMP "esprit-$Version.zip"

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Invoke-WebRequest $Asset.browser_download_url -OutFile $Temp
Expand-Archive $Temp -DestinationPath $InstallDir -Force
Remove-Item $Temp -Force

$Path = [Environment]::GetEnvironmentVariable("Path", "User")

if ($Path -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$Path;$InstallDir",
        "User"
    )
}

Write-Host "Esprit $Version installed."
