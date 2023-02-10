# Rebuild project on *.rs, *.frag or *.vert file change
watch:
    watchexec -e rs,frag,vert -r -w {{justfile_directory()}}/src just build

# This will start server at http://localhost:3010
sync:
    npx browser-sync start --server --files "./pkg/*" --port 3010

# Run wasm-pack build
build:
    cd {{justfile_directory()}}; wasm-pack build --no-typescript --target web
