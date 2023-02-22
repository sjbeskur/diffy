# Diffy

''' bash
rustup target list
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu  
'''

Build for aarch64

``` bash
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=/usr/bin/aarch64-linux-gnu-gcc
cargo build --target aarch64-unknown-linux-gnu --release
```

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-unknown-linux-gnu --release
```

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target aarch64-unknown-linux-gnu --release
```