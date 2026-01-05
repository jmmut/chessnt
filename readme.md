# MY_CRATE_NAME

The first thing to do after cloning this template repo is `./rename_crate.sh new-name`, where new-name
is the name you want for this project. For `new-name`, I recommend to used kebab-case rather than snake_case, because
itch.io only supports kebab-case URLs, and the github actions workflow would need some case conversion
code for running butler. With a kebab-case name, all should work after running rename_crate.sh.

After this you still have to add a BUTLER_API_KEY secret to the github repo so that the github workflow
can deploy to itch.io. If the github workflow still fails after this, try uploading manually an initial html5 build.

## Running this project

Clone this repo, then [Install rust](https://www.rust-lang.org/tools/install), then do `cargo run --release`.

