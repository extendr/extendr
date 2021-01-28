function try-cargo {
    param(
        [Parameter(Position = 0, ValueFromRemainingArguments)]
        [String[]]
        $CargoArgs,
        [String]
        $ActionName
    )
    try {    
        echo "::group::$ActionName"
        echo "Running cargo $CargoArgs"
        cargo $CargoArgs
    }
    catch {
        $err_msg = "::error::$ActionName cargo failed with code $LASTEXITCODE (args: $CargoArgs)"
        Write-Error -Message "$err_msg" -ErrorAction Stop 
    }
    finally {
        echo "::endgroup::"
    }
    
}
