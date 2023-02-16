# rayon test
Testing this PR, that allows us to run `rayon` on Wasm without threads: https://github.com/rayon-rs/rayon/pull/1019

We benchmark summing a `Vec<u64>` with a for-loop, with `iter().sum()` and with `par_iter().sum()`.

## Results
I wanted to see wether or not using `par_iter()` without any actual threading would add a lot of overhead.

Tested on an M1 MacBook Pro.

### Native
We see `par_iter` winning, as expected:

![image](https://user-images.githubusercontent.com/1148717/219310956-4d957648-f4c4-41a2-9d6e-5aaf179ab953.png)

### Wasm in Firefox
We see that `iter()` and `par_iter()` performs almost exactly the same, with a very minimal overhead cost for `par_iter()`:

![image](https://user-images.githubusercontent.com/1148717/219310821-fdc15b74-b1c7-4ada-a24b-0147dad02cb8.png)

# Running it yourself
## Natively
`cargo run --release`

## Web

1. Install Trunk with `cargo install --locked trunk`.
2. Run `trunk serve --release` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
3. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.
