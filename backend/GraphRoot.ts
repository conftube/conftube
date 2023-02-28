import {buildSchema} from "graphql";
import {addVideo, searchVideos, AddVideoInput, videos} from "./videos";

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
    
    type PaginatedVideos {
        first: Int
        offset: Int
        total: Int
        results: [Video]
    }

    type Query {
        videos(id: ID, first: Int, offset: Int): PaginatedVideos
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
    videos: videos,
    addVideo: ({input}: { input: AddVideoInput }) => {
        return addVideo(input)
    },
}
