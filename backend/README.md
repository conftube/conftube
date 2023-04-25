# conftube backend

The backend of conftube is built with Rust, actix-web, async-graphql and diesel on top of a PostgreSQL database.

## Development

Install required tools:

```shell
# see https://diesel.rs/guides/getting-started for details
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-watch
```

Copy `.env.dist` and rename it to `.env` and fill in the missing details. After that, start the development server:

```sh
cargo watch -x run
```

This will compile the code, run migrations, start the development server as well as watch & recompile for changes.

## Database changes

`conftube` uses [diesel](https://diesel.rs) under the hood, so changes in the schema are being run through migrations.
Run the following to generate a new migration:

```shell
diesel migration create <name>
```
