# Sudoku Solver with GUI

Wanted to learn some Rust and wrote little Sudokusolver with little GUI for fun. It's quite fast and managed to solve everything I threw in.

## In Linux:

Compile with command:
```
cargo run --features gtk_3_22
```
## In Windows:

First:
```
rustup target add x86_64-pc-windows-gnu
```
Then compile with command:
```
cargo run --features gtk_3_22 --target=x86_64-pc-windows-gnu
```
If you get an error, then go to:
Control Panel → System and Security → System → Advanced system settings → Environment variables and add new variable and try again:
```
PKG_CONFIG_ALLOW_CROSS=1
rustup toolchain install stable-x86_64-pc-windows-gnu
```
https://github.com/gtk-rs/gtk/issues/44
https://www.reddit.com/r/rust/comments/6rerw5/tutorial_cross_compiling_a_gtk_program_from_linux/
http://www.mingw.org/wiki/howto_install_the_mingw_gcc_compiler_suite


Hafe fun!
