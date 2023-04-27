import { useQuery } from "@apollo/client";
import { ReactElement } from "react";
import { GET_SEARCHVIDEO } from "./queries";

export default function SearchVideo({
  filterQuery,
}: {
  filterQuery: string;
}): ReactElement | null {
  const { loading, error, data } = useQuery(GET_SEARCHVIDEO, {
    errorPolicy: "all",
    skip: filterQuery.length <= 2,
    variables: { filterQuery },
  });

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error : {error.message}</p>;
  if (error === 401) <a href="localhost:3000/login"></a>;
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
