import "./App.css";
import { useQuery, gql } from "@apollo/client";

export default function App() {
  const GET_SEARCHVIDEO = gql`
    query GetSearchVideo {
      searchVideos("JavaScript") {
        title
        description
        platform
        thumbnailUrl
        id
        publishedAt
      }
    }
  `;

  function DisplayVideo() {
    const { loading, error, data } = useQuery(GET_SEARCHVIDEO);
    console.log(data);

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error : {error.message}</p>;

    return data.map(
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
    );
  }

  return (
    <div>
      <h2>My first Apollo app 🚀</h2>
      <DisplayVideo />
    </div>
  );
}
