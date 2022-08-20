# Simple Rust 🦀 server

Run it from root directory using:
```bash
cargo run
```

## env variables 

List of env variables

| Variable name    | Description                                           |
| ---------------- | ----------------------------------------------------- |
| `SERVER_ADDRESS` | IP and Port of your server, overrides system variable |

Also, consider looking into `.cargo/config.toml`


## Dev recommendations:

1) Instal [`cargo install cargo-watch`](https://crates.io/crates/cargo-watch) and watch for changes when developing with `cargo watch -x src`
2) Use [`rust-clippy`](https://github.com/rust-lang/rust-clippy) linter to make your code even better
