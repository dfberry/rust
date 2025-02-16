# Axum Diesel R2D2 pool with Neon postgres DB

## To run server

```
RUST_LOG=info cargo run
```


## Get R2D2 error

Get this error on startup and every 10 minutes but don't get it when requests come in and are processed by diesel

```
ERROR r2d2: unnamed prepared statement does not exist
``