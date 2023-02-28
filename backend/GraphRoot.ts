import {buildSchema} from "graphql";
import {addVideo, searchVideos, AddVideoInput, videos} from "./videos";
import {profile} from "./user";

export const schema = buildSchema(`
    type User {
        id: ID!
        email: String
        givenName: String
        familyName: String
        picture: String
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
        profile: User
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

export const resolvers = {
    searchVideos: searchVideos,
    videos: videos,
    profile: profile,
    addVideo: ({input}: { input: AddVideoInput }) => {
        return addVideo(input)
    },
}
