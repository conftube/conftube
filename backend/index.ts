import express, {Express} from 'express';
import dotenv from 'dotenv';
import {graphqlHTTP} from 'express-graphql';
import {root, schema} from "./GraphRoot";

dotenv.config();

const app: Express = express();
const port = process.env.PORT ?? '8080';
const hostname = process.env.HOSTNAME ?? '0.0.0.0';

app.use('/graphql', graphqlHTTP({
    schema: schema,
    rootValue: root,
    graphiql: true,
}));

app.listen(parseInt(port), hostname, () => {
    console.log(`Server is now running on http://localhost:${port}`);
});
