CLS

# _______POWERSHELL______
$Env:RUST_BACKTRACE=1
# $Env:RUST_BACKTRACE=full

# _______CMD_____________
# SET RUST_BACKTRACE=1
# SET RUST_BACKTRACE=full

# wasm-pack build --dev --target web --out-name app --out-dir ./static crate # LAST COMPILE COMMAND
wasm-pack build --dev --target web crate --out-dir ../dist
# wasm-pack build --target no-modules --out-name app --out-dir ./static 

# erase .\static\*.ts
# erase .\static\package.json
# erase .\static\wasm_bg.d.ts
# erase .\static\wasm.d.ts

Write-Host "PowerShell Build Done"