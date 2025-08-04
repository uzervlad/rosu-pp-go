mkdir -p bindings
cargo build -rq
cargo test --quiet

mkdir -p lib
cp target/release/librosu_pp_go.a lib/
cp bindings/rosu_pp_go.h lib/
