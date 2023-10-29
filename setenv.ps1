# $env:R_HOME = "C:\Program Files\R\R-devel"
$env:R_HOME = "C:\Users\minin\scoop\apps\r\current"
$env:MINGW_ROOT = "C:\rtools43\x86_64-w64-mingw32.static.posix"
# $env:LIBRSYS_LIBCLANG_INCLUDE_PATH = "${env:MINGW_ROOT}\include"
$env:LIBRARY_PATH = "libgcc_mock"
# $env:PATH = "%R_HOME%\bin\x64;%MINGW_ROOT%\bin;C:\rtools42\usr\bin;${env:PATH}"
# [System.Environment]::SetEnvironmentVariable("PATH", $env:PATH, "User")

# $env:LIBRSYS_BINDINGS_OUTPUT_PATH = "bindings"
[System.Environment]::SetEnvironmentVariable("MINGW_ROOT", $env:MINGW_ROOT, "User")
[System.Environment]::SetEnvironmentVariable("R_HOME", $env:R_HOME, "User")
