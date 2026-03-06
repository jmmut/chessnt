
# License of Chessn't

This game is licensed under the [GPL3](licenses/GPL3_Chessnt.txt).

The informal intention is that if you make a commercial derivative,
you have to license your project as GPL3 too.
Some of the consequences are that you have to distribute source code copies of your project,
and make a visible notice explaining that your project is a derivative of this project,
and which modification you made.


The fonts used in this game are licensed under OFL:
- [Lilita](licenses/OFL_Lilita.txt)
- [Unbounded](licenses/OFL_Unbounded.txt)
- [JetBrains_Mono](licenses/OFL_JetBrains_Mono.txt)

The source code dependencies are licensed under MIT, Apache 2.0, or BSD3.
See [LICENSE-3rdparty.csv](licenses/LICENSE-3rdparty.csv) for details.


### Regenerate 3rd party license report

```
cargo install dd-rust-license-tool
dd-rust-license-tool --config licenses/license-tool.toml write
mv LICENSE-3rdparty.csv licenses/
```
