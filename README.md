First install sqlx-cli, for example:
```sh
cargo install sqlx-cli --no-default-features --features postgres,rustls
```
(The sqlx-cli is documented at https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

Next, copy `example.env` to `.env` and edit to match your local configuraiton.

With postgres installed and `.env` properly configured, create a database and schema as follows:
```sh
sqlx database create
sqlx migrate run
```

Note, this is a very simplistic POC to test using [Trillium](https://trillium.rs/) as the underlying framework for the [Neighbor CMS](https://github.com/jeremyandrews/neighbor). All functionality has not yet been ported.