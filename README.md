# newsletter-server

### Install sqlx-cli to run DB migration:

```shell
cargo install --version=0.8.5 sqlx-cli --no-default-features --features postgres
```

### Install psql

- Follow [this](https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows)

### add .sqlx folder with cached query data for offline SQLX mode

```shell
cargo sqlx prepare --workspace -- --all-targets
```
