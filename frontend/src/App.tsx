import "./App.css";
import { useQuery, gql } from "@apollo/client";
import { ChangeEvent, ReactElement, useState, useMemo } from "react";
import _ from "lodash";

export default function App() {
  function DisplayVideo({
    filterQuery,
  }: {
    filterQuery: string;
  }): ReactElement | null {
    const GET_SEARCHVIDEO = gql`
      query GetSearchVideo {
        searchVideos(query: "${filterQuery}") {
          title
          description
          platform
          thumbnailUrl
          id
          publishedAt
        }
      }
    `;

    const { loading, error, data } = useQuery(GET_SEARCHVIDEO, {
      variables: GET_SEARCHVIDEO,
    });

    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error : {error.message}</p>;

    return data
      ? data.searchVideos.map(
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
        )
      : null;
  }
  const [filterQuery, setFilterQuery] = useState<number | string | any>("");

  const debouncedChangeHandler = useMemo(
    () => _.debounce(handleChange, 300),
    []
  );

  function handleChange(e: ChangeEvent<HTMLInputElement>) {
    setFilterQuery(e.target.value);
  }

  return (
    <div className="App-header">
      <h2>Conftube</h2>
      <div>
        <div className="search_container">
          <input
            type="text"
            placeholder="Search for talk"
            onChange={debouncedChangeHandler}
          />
        </div>
        <DisplayVideo filterQuery={filterQuery} />
      </div>
    </div>
  );
}
