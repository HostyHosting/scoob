#!/usr/bin/env pwsh
# Based on the Deno Installer: https://github.com/denoland/deno_install/blob/master/install.ps1

$ErrorActionPreference = 'Stop'

if ($v) {
  $Version = "v${v}"
}
if ($args.Length -eq 1) {
  $Version = $args.Get(0)
}

$ScoobInstall = $env:SCOOB_INSTALL
$BinDir = if ($ScoobInstall) {
  "$ScoobInstall\bin"
} else {
  "$Home\.scoob\bin"
}

$ScoobZip = "$BinDir\scoob.zip"
$ScoobExe = "$BinDir\scoob.exe"
$Target = 'x86_64-pc-windows-msvc'

# GitHub requires TLS 1.2
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$ScoobUri = if (!$Version) {
  "https://github.com/hostyhosting/scoob/releases/latest/download/scoob-${Target}.zip"
} else {
  "https://github.com/hostyhosting/scoob/releases/download/${Version}/scoob-${Target}.zip"
}

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

Invoke-WebRequest $ScoobUri -OutFile $ScoobZip -UseBasicParsing

if (Get-Command Expand-Archive -ErrorAction SilentlyContinue) {
  Expand-Archive $ScoobZip -Destination $BinDir -Force
} else {
  if (Test-Path $ScoobExe) {
    Remove-Item $ScoobExe
  }
  Add-Type -AssemblyName System.IO.Compression.FileSystem
  [IO.Compression.ZipFile]::ExtractToDirectory($ScoobZip, $BinDir)
}

Remove-Item $ScoobZip

$User = [EnvironmentVariableTarget]::User
$Path = [Environment]::GetEnvironmentVariable('Path', $User)
if (!(";$Path;".ToLower() -like "*;$BinDir;*".ToLower())) {
  [Environment]::SetEnvironmentVariable('Path', "$Path;$BinDir", $User)
  $Env:Path += ";$BinDir"
}

Write-Output "Scoob was installed successfully to $ScoobExe"
Write-Output "Run 'scoob --help' to get started"
