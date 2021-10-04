



Fix:
"error: linking with link.exe failed: exit code: 1"

To solve it:

rustup toolchain install stable-x86_64-pc-windows-gnu

then

rustup default stable-x86_64-pc-windows-gnu

and

cargo build
  Compiling hello v0.1.0 (C:\Users\leke\dev\rust\hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.66s

