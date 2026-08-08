$InstallDir = "$env:LOCALAPPDATA\Esprit"

if (Test-Path $InstallDir) {
    Remove-Item $InstallDir -Recurse -Force
}

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$NewPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir }) -join ';'
[Environment]::SetEnvironmentVariable("Path", $NewPath, "User")

Write-Host "Esprit uninstalled."
