# [Rust](https://www.rust-lang.org) web app template with [actix web 2.0](https://actix.rs)

## Usage
Use [cargo generate](https://github.com/ashleygwilliams/cargo-generate) to clone this template:
```bash
cargo generate --git https://github.com/thisKai/actix-web-boilerplate.git --name my-project
cd my-project
```

## [Auto-Reloading Development Server](https://actix.rs/docs/autoreload/)

### Requirements
- [systemfd](https://crates.io/crates/systemfd)
- [cargo-watch](https://crates.io/crates/cargo-watch)

```bash
cargo install systemfd cargo-watch
```

To run the development server:
```bash
cargo xtask dev
```

## Heroku
This template works with the [emk/rust](https://github.com/emk/heroku-buildpack-rust) buildpack:
```bash
heroku create --buildpack emk/rust
```
