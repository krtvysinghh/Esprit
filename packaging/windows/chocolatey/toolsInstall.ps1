$ErrorActionPreference = 'Stop'

$version = '0.1.0'
$url = "https://github.com/krtvysinghh/Esprit/releases/download/v$version/esprit-v$version-windows-x86_64.zip"

$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$zip = Join-Path $toolsDir 'esprit.zip'

Get-ChocolateyWebFile `
  -PackageName 'esprit' `
  -FileFullPath $zip `
  -Url $url `
  -Checksum 'REPLACE_WITH_RELEASE_SHA256' `
  -ChecksumType 'sha256'

Get-ChocolateyUnzip `
  -FileFullPath $zip `
  -Destination $toolsDir
