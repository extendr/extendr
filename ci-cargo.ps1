function ci-cargo {

    param(
        [Parameter(Position = 0, ValueFromRemainingArguments)]
        [String[]]
        $CargoArgs,
        [String]
        $ActionName
    )


    try {
        Write-Output "::group::$ActionName"
        $CargoArgs = $CargoArgs | Where-Object { -not [string]::IsNullOrWhiteSpace($_) }
        Write-Output "Running cargo $CargoArgs"
        cargo $CargoArgs
        if ($LASTEXITCODE -ne 0) {
            throw $LASTEXITCODE
        }
    }
    catch {
        if ($ActionName -ne $null -and $ActionName -ne "") {
            $ActionName = "'$ActionName': "
        }
        $errMsg = "$($ActionName)cargo failed with code $LASTEXITCODE (args: $CargoArgs)"
        Write-Output "::error::$errMsg"
        Write-Error -Message "$errMsg" -ErrorAction Stop
    }
    finally {
        Write-Output "::endgroup::"
    }

    <#
        .SYNOPSIS
        Runs cargo with specified args, adapting error handling and output to CI.

        .DESCRIPTION
        Runs cargo in a `try` block, catches exceptions and non-zero exit codes.
        Explicitly logs the beginning and the end of cargo execution, as well as the error message.

        .PARAMETER CargoArgs
        Arguments passed to cargo, as-is.
        Note that `--` separator is handled by powershell itself,
        so it should be wrapped in quotes `'--'` and passed as string.

        .PARAMETER ActionName
        Optional string that is used to format logs and error messages.

        .INPUTS
        None. You cannot pipe objects.

        .OUTPUTS
        No explicit output.

        .EXAMPLE
        PS> ci-cargo --version
            ::group::
            Running cargo --version
            cargo 1.49.0 (d00d64df9 2020-12-05)
            ::endgroup::

        .EXAMPLE
        PS> ci-cargo -ActioName "Build"  build

        .EXAMPLE
        PS> ci-cargo +stable-x86_64-pc-windows-gnu test --features tests-all --target i686-pc-windows-gnu '--' --nocapture -ActionName "Called from documentation"

        .LINK
        Used by: https://github.com/extendr/extendr
    #>
    
}
