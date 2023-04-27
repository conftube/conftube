import "./App.css";
import { ChangeEvent, useState, useMemo } from "react";
import _ from "lodash";
import SearchVideo from "./components/SearchVideo";
import { Card, CardMedia } from "@mui/material";

export default function App() {
  const [filterQuery, setFilterQuery] = useState<number | string | any>("");

  const debouncedChangeHandler = useMemo(
    () => _.debounce(handleChange, 300),
    []
  );

  function handleChange(e: ChangeEvent<HTMLInputElement>) {
    setFilterQuery(e.target.value);
  }

  //TODO: Auth handeln, path: localhost:3000/login

  // function errorHandler(callback: any, errorCode = 401) {
  //   return callback.catch((e: any) => {
  //     if (errorCode === e.statusCode) {
  //       ("/login");
  //     }
  //   });
  // }

  return (
    <Card variant="outlined">
      <CardMedia
        component="img"
        height="140"
        image="./assets/alexandre-pellaes-6vAjp0pscX0-unsplash.jpg"
        alt="technical talk"
      />
      <h1>Conftube</h1>
      <div>
        <div className="search_container">
          <input
            type="text"
            placeholder="Search for talk"
            onChange={debouncedChangeHandler}
          />
        </div>
        <SearchVideo filterQuery={filterQuery} />
      </div>
    </Card>
  );
}
