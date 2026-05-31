# Chessn't

Early prototype of a game where you play chess and can cheat.

## Play

You can play the web version at https://jmmut.itch.io/chessnt 
or download native executables for Linux, Mac or Windows.

## Compiling and running this project

Clone this repo, then [Install rust](https://www.rust-lang.org/tools/install), then do `cargo run --release`.

### Cross compiling

[The github workflow](.github/workflows/release.yml) cross compiles to windows,
mac and web assembly (also regular-compiles linux).

To cross compile, follow the steps of the workflow. You might find problems:

#### Troubleshooting: linux -> windows: linker not found

If you try to compile for windows from a linux machine and it fails with:
```
error: linker `x86_64-w64-mingw32-gcc` not found
```
you might need to install a mingw linker and tell cargo about it:
```
sudo apt install mingw-w64
echo '
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
' >> ~/.cargo/config
```

then, `cargo b -r --target x86_64-pc-windows-gnu` should work.

Btw, running the windows executable in linux works in my machine:
`wine target/x86_64-pc-windows-gnu/release/chessnt.exe`
