import "./App.css";
import Search from "./components/Search";
import Content from "./components/Content";
import { useQuery, gql } from "@apollo/client";

const GET_VIDEOS = gql`
  query Video {
    video {
      titel
      id
    }
  }
`;

const { data } = useQuery(GET_VIDEOS);

export default function App() {
  return (
    <div className="App">
      <header className="App-header">
        <div>{data.video.titel}</div>
        <Search />
        <Content />
      </header>
    </div>
  );
}
