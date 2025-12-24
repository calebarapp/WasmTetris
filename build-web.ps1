param(
    [string]$Configuration = "release",
    [string]$Target = "wasm32-unknown-unknown",
    [string]$PublishDir = "publish"
)

# 1) Build WASM
cargo build --target $Target --$Configuration
if ($LASTEXITCODE -ne 0) {
    Write-Error "Cargo build failed."
    exit 1
}

# 2) Ensure publish folder exists
New-Item -ItemType Directory -Path $PublishDir -Force | Out-Null

# 3) Copy WASM (change 'your_game' to your crate name)
$crateName = "wasm_tetris"              # <-- CHANGE THIS
$wasmName  = "$crateName.wasm"
$wasmSrc   = "target\$Target\$Configuration\$crateName.wasm"
$wasmDest  = Join-Path $PublishDir $wasmName
Copy-Item $wasmSrc $wasmDest -Force

# 4) Copy static files
Copy-Item .\index.html       (Join-Path $PublishDir "index.html")       -Force
Copy-Item .\mq_js_bundle.js  (Join-Path $PublishDir "mq_js_bundle.js")  -Force

Write-Host "Published to '$PublishDir'"
