# conftube backend

The backend of conftube is built with express, express-graphql and prisma on top of a PostgreSQL database.

## Development

Install required tools:

```shell
# see https://diesel.rs/guides/getting-started for details
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-watch
```

Run database migrations & start the development server:

```sh
diesel migration run && cargo watch -x run
```

This will compile the code, start the development and watch & recompile for changes.

## Database changes

`conftube` uses [diesel](https://diesel.rs) under the hood, so changes in the schema are being run through migrations. Run the following to generate a migration:

```shell
diesel migration create <name>
```
