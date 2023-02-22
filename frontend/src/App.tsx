import "./App.css";
import Search from "./components/Search";
import Content from "./components/Content";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <Search />
        <Content />
      </header>
    </div>
  );
}

export default App;
