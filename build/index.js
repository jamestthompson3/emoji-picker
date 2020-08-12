import debounce from "./debounce.js";

(function() {
  const tauri = window.__TAURI__;
  tauri.event.listen("result", appendResult);
  const inputBox = document.getElementById("search");
  const resultsBox = document.getElementById("results");
  let previousSearch;
  const search = ({ target: { value } }) => {
    if (value !== previousSearch) {
      console.log("no prev value: ", previousSearch, value);
      tauri.event.emit("search", value);
      previousSearch = value;
    }
  };
  inputBox.onkeyup = debounce(search, 250);
  inputBox.focus();

  function appendResult({ payload }) {
    const emojiContainer = document.createElement("p");
    emojiContainer.textContent = payload;
    resultsBox.appendChild(emojiContainer);
  }
})();
