import express, {Express} from 'express';
import dotenv from 'dotenv';
import {graphqlHTTP} from 'express-graphql';
import {resolvers, schema} from "./GraphRoot";
import {auth, ConfigParams, requiresAuth} from 'express-openid-connect';

dotenv.config();

const app: Express = express();
const port = parseInt(process.env.PORT ?? '8080');
const hostname = process.env.HOSTNAME ?? '0.0.0.0';

export type AuthenticatedUser = {
    email: string,
    given_name: string,
    family_name: string,
    picture: string
}

export type Context = {
    user: AuthenticatedUser
}

const config: ConfigParams = {
    authRequired: false,
    auth0Logout: true,
    secret: process.env.APPLICATION_SECRET,
    baseURL: process.env.BASE_URL,
    clientID: process.env.AUTH0_CLIENT_ID,
    issuerBaseURL: process.env.AUTH0_ISSUER_BASE_URL,
};

app.use(auth(config)); // auth router attaches /login, /logout, and /callback routes to the baseURL

app.use('/graphql', requiresAuth(), graphqlHTTP(async  (req) => ({
    schema,
    rootValue: resolvers,
    graphiql: true,
    context: {
        // @ts-ignore
        user: req.oidc.user
    }
})));

app.listen(port, hostname, () => {
    console.log(`Server is now running on ${hostname}:${port}`);
});
