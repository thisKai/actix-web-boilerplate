# Rust web app boilerplate with actix web 2.0

## [Auto-Reloading Development Server](https://actix.rs/docs/autoreload/)

### Requirements
- [systemfd](https://crates.io/crates/systemfd)
- [cargo-watch](https://crates.io/crates/cargo-watch)

```bash
cargo install systemfd cargo-watch
```

To run the development server:
```bash
systemfd --no-pid -s http::$PORT -- cargo watch -x run
```
Or use the included bash script:
```bash
./dev
```

## Heroku
This template works with the [emk/rust](https://github.com/emk/heroku-buildpack-rust) buildpack:
```bash
heroku create --buildpack emk/rust
```
