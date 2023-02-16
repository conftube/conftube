import "./App.css";
import Search from "./components/Search";
import Content from "./components/Content";

function App() {
  return (
    <div className="App">
      <header className="header">ConfTube</header>
      <Search />
      <Content />
    </div>
  );
}

export default App;
