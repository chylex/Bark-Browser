Set-Location (Split-Path $PSScriptRoot -Parent)

if (!(Test-Path "out")) {
    mkdir "out"
}

cargo build --release

$version = & "target\release\bark.exe" --version
$arch = switch ((Get-WMIObject -Class Win32_Processor).Architecture) {
    0 { "x86" }
    1 { "mips" }
    2 { "alpha" }
    3 { "ppc" }
    5 { "arm" }
    6 { "ia64" }
    9 { "x64" }
    12 { "arm64" }
    DEFAULT { "unknown" }
}

Compress-Archive -Force -Path "target\release\bark.exe" -DestinationPath "out\bark-${version}-windows-${arch}.zip"
