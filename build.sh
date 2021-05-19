# This was working on aarch64 macos
#cargo build
#gcc main.c -l v0_baseline -L  target/debug -o hello


RUSTFLAGS="-C target-feature=+crt-static" cargo build --lib --release  
clang -Og main.c -l v0_baseline -L  target/release -o hello  -lpthread -ldl 
