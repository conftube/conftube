import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import { buildSchema } from 'graphql';
import { graphqlHTTP } from 'express-graphql';

const videos = [
  {
    title: "How not to learn GraphQL",
    url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  },
  {
    title: "Just another video",
    url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
  },
];

const schema = buildSchema(`
    type User {
        name: String,
        email: String,
        password: String,
    },

    type Video {
        title: String,
        description: String,
        owner: String,
        url: String,
        uploaded_at: String
        uploader: User
    }

    type Rating {
        user: User,
        video: Video
    }

    type Query {
        videos: [Video],
        searchVideos(title: String): [Video]
    }
`);

const root = {
    videos: () => {
        return videos;
    },
    searchVideos: ({ title }) => {
        return videos.filter(e => e.title == title);
    }
}

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
