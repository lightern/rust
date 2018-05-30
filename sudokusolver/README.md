# Sudoku Solver with GUI

Wanted to learn some Rust and wrote little Sudokusolver with little GUI (GTK) for fun. It's quite fast and managed to solve everything I threw in.

## In Linux:

Compile with command:
```
cargo run --features gtk_3_22
```
## In Windows:

1) First go to http://www.msys2.org/ and install msys (x86_64) and check to run it after install (needed to install some stuff):

2) Run these commands inside MSYS (not Windows cmd!) you just opened and accept when asked to confirm (this will install some stuff for compiler to use):
```
pacman -S mingw-w64-x86_64-gtk3
pacman -S mingw-w64-x86_64-toolchain
```
3) Make the needed environment variable linkings inside Window's own cmd:
```
SET GTK_LIB_DIR=C:\msys64\mingw64\lib
SET PATH=%PATH%;C:\msys64\mingw64\bin
SETX GTK_LIB_DIR %GTK_LIB_DIR%
SETX PATH %PATH%
```
4) Add windows-gnu compability to rustup with in cmd:
```
rustup target add x86_64-pc-windows-gnu
```
5) Then compile with command in cmd (in cmd and right project folder):
```
cargo rustc --release --features gtk_3_22 --target=x86_64-pc-windows-gnu -- -C link-args=-mwindows
```

More info on Windows + Rust + GTK combination:
http://gtk-rs.org/docs/requirements.html
