import {buildSchema} from "graphql";
import {searchVideos} from "./videos";

export const schema = buildSchema(`
    type User {
        id: String
        email: String
    }

    type VideoSearchResult {
        id: ID!
        platform: String # could be either 'youtube' or anything else really
        title: String
        description: String
        publishedAt: String
    }
    
    type Query {
        searchVideos(title: String): [VideoSearchResult]
    }
`);

export const root = {
    searchVideos: searchVideos
}
