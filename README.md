# rust-webgl-wasm-testing
Trying out rust and webgl using wasm. Probably won't get to finishing it but I wanted to try :tm:.

# Development Setup

You obviously need cargo/rustc, also optionaly maid as a task runner. If you don't have maid you can run the folowing:

`yarn global add maid`

If you don't want to bother just read the [maidfile](../blob/master/maidfile.md) for all the tasks then you can just run the commands manually.

You will need the following cargo features installed:

`cargo install wasm-pack cargo-watch basic-http-server`

Finally, build the dependencies:

`maid build`
