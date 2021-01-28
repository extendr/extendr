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
        if($LASTEXITCODE -ne  0) {
            throw $LASTEXITCODE
        }
    }
    catch {
        if ($ActionName -ne $null -and $ActionName -ne "") {
            $ActionName = "'$ActionName':"
        }
        $err_msg = "::error::$ActionName cargo failed with code $LASTEXITCODE (args: $CargoArgs)"
        echo $err_msg
        Write-Error -Message "$err_msg" -ErrorAction Stop 
    }
    finally {
        echo "::endgroup::"
    }
    
}
