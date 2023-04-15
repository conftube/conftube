# conftube backend

The backend of conftube is built with Rust, actix-web, async-graphql and diesel on top of a PostgreSQL database.

## Development

Install required tools:

```shell
# see https://diesel.rs/guides/getting-started for details
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-watch
```

Copy `.env.dist` to `.env` and fill in the missing details. Then run database migrations & start the development server:

```sh
diesel migration run && cargo watch -x run
```

This will compile the code, start the development server as well as watch & recompile for changes.

## Database changes

`conftube` uses [diesel](https://diesel.rs) under the hood, so changes in the schema are being run through migrations.
Run the following to generate a new migration:

```shell
diesel migration create <name>
```
