mkdir -p bindings
cargo build -rq
cargo test --quiet

mkdir -p lib
# idk just copy all of them
cp target/release/librosu_pp_go.d lib/
cp target/release/librosu_pp_go.rlib lib/
cp target/release/rosu_pp_go.d lib/
cp target/release/rosu_pp_go.dll lib/
cp target/release/rosu_pp_go.dll.lib lib/
cp target/release/librosu_pp_go.so lib/
cp bindings/rosu_pp_go.h lib/
