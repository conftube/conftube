# conftube backend

The backend of conftube is built with express, express-graphql and prisma on top of a PostgreSQL database.

## Development

Start the development server:

```sh
npm run dev
```

This will compile the code, start the development and watch & recompile for changes.

## Database changes

`conftube` uses Prisma under the hood, so changes in the schema need to have a migration generated from the existing
schema as well as code generation is required. Run the following after changing `prisma/schema.prisma`:

```shell
npx prisma migrate dev --name <a-descriptive-name-for-the-change>
npx prisma generate
```
