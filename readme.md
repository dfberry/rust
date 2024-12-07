# Rust

* Code: [https://github.com/dfberry/rust.git](https://github.com/dfberry/rust.git)
* [YouTube playlist](https://www.youtube.com/playlist?list=PLAQX7qAUlTDh6RoD-S8HhJdSF0mEEXuYV)

## Reference

[The Rust Programming Language for 2021](https://www.amazon.com/gp/aw/d/1718503105/)

## 001 Hello world with Cargo

* Dev container for Rust
* Monorepo 
* Debugging
* Example crate

[YouTube video](https://youtu.be/uEPa3lWKPMs)

## 002 Guessing game

### Cargo build warning: resolver

When running Cargo build, get warning before build completes:

```
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
   Compiling libc v0.2.152
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.12
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling crate_002_guessing_game v0.1.0 (/workspaces/rust/crates/crate_002_guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.92s
```

Fix: update root `Cargo.toml` with resolver of `2`

```
[workspace]
members = ["crates/*"]
resolver = "2"
```

### Version for rand doesn't update?

No idea why this happened. 

## Examples

* [Rust axum social auth for cron](https://github.com/Cucharoth/crono-server)
* [Cron scheduler](https://crates.io/crates/tokio-cron-scheduler)
* [Cron job blog](https://tpbabparn.medium.com/feasibility-of-implementing-cronjob-with-rust-programming-language-186eaed0a7d8)
* [Example scheduler](https://github.com/IgnisDa/ryot/blob/main/crates/background/src/lib.rs)
