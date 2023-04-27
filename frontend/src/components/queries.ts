import { gql } from "@apollo/client";

export const GET_SEARCHVIDEO = gql`
  query getSearchVideo($filterQuery: String!) {
    searchVideos(query: $filterQuery) {
      title
      description
      platform
      thumbnailUrl
      id
      publishedAt
    }
  }
`;
