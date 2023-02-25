import {buildSchema} from "graphql";
import {addVideo, searchVideos, AddVideoInput} from "./videos";

export const schema = buildSchema(`
    type User {
        id: String
        email: String
    }
    
    type Video {
        id: ID!
        platform: String
        title: String
        description: String
        thumbnailUrl: String
        publishedAt: String
    }
    
    type Query {
        searchVideos(query: String): [Video]
    }
    
    input AddVideoInput {
        id: ID!
        platform: String
    }
    
    type Mutation {
        addVideo(input: AddVideoInput): Video
    }
`);

export const root = {
    searchVideos: searchVideos,
    addVideo: ({input}: { input: AddVideoInput }) => {
        return addVideo(input)
    },
}
