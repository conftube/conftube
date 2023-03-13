import "./App.css";
import { useQuery, gql } from "@apollo/client";
import {ReactElement} from "react";

export default function App() {
  function DisplayVideo(): ReactElement | null {
    const GET_SEARCHVIDEO = gql`
      query GetSearchVideo {
        searchVideos(query: "JavaScript") {
          title
          description
          platform
          thumbnailUrl
          id
          publishedAt
        }
      }
    `;

    const { loading, error, data } = useQuery(GET_SEARCHVIDEO);
    console.log({data});

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error : {error.message}</p>;

    return data ? data.searchVideos.map(
      ({
        title,
        description,
        platform,
        thumbnailUrl,
        id,
        publishedAt,
      }: {
        title: string;
        description: string;
        platform: string;
        thumbnailUrl: string;
        id: number;
        publishedAt: string;
      }) => (
        <div key={id}>
          <h3>{title}</h3>
          <video width="400" height="250" src={`${thumbnailUrl}`} />
          <br />
          <p>{description}</p>
          <p>{platform}</p>
          <p>{publishedAt}</p>
          <br />
        </div>
      )
    ) : null;
  }

  return (
    <div className="App-header">
      <h2>My first Apollo app 🚀</h2>
      <DisplayVideo />
    </div>
  );
}
