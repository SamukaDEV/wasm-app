@ECHO OFF
CLS

REM _______CMD_____________
SET RUST_BACKTRACE=1
REM SET RUST_BACKTRACE=full

wasm-pack build --target web --out-name app --out-dir ./static 
REM wasm-pack build --target no-modules --out-name app --out-dir ./static 

REM erase .\static\*.ts
REM erase .\static\package.json
REM erase .\static\wasm_bg.d.ts
REM erase .\static\wasm.d.ts

ECHO "CMD Build Done"