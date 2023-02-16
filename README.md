# rayon test
Testing this PR, that allows us to run `rayon` on Wasm without threads: https://github.com/rayon-rs/rayon/pull/1019

### Testing it locally
`cargo run --release`

### Testing it on the web

1. Install Trunk with `cargo install --locked trunk`.
2. Run `trunk serve --release` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
3. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.
