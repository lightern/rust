## Sudoku Solver with GUI

Wanted to learn some Rust and wrote little Sudokusolver with little GUI for fun. It's quite fast and managed to solve everything I threw in.

# In linux:

Compile with command:
cargo run --features gtk_3_22

# In Windows:

First:
rustup target add x86_64-pc-windows-gnu

Then compile with command:
cargo run --features gtk_3_22 --target=x86_64-pc-windows-gnu

Currently still gives:
Cross compilation detected. Use PKG_CONFIG_ALLOW_CROSS=1 to override

Working on it...


Hafe fun!
