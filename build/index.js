import debounce from "./debounce.js";

(function() {
  const tauri = window.__TAURI__;
  tauri.event.listen("result", appendResult);
  tauri.event.listen("loadCache", appendInitial);
  const inputBox = document.getElementById("search");
  const resultsBox = document.getElementById("results");
  let previousSearch;
  const search = ({ target: { value } }) => {
    if (value !== previousSearch) {
      tauri.event.emit("search", value);
      previousSearch = value;
    }
  };
  inputBox.onkeyup = debounce(search, 250);

  inputBox.focus();

  function appendInitial({ payload }) {
    const { recent } = JSON.parse(payload);
    appendResult({ payload: recent });
  }

  function appendResult({ payload }) {
    // clear current list if it exists
    while (resultsBox.firstChild) {
      resultsBox.removeChild(resultsBox.lastChild);
    }
    for (const emoji of payload) {
      const emojiContainer = document.createElement("p");
      emojiContainer.textContent = emoji;
      emojiContainer.onclick = () => tauri.event.emit("select", emoji);
      emojiContainer.classList.add("pointer", "focus-ring");
      resultsBox.appendChild(emojiContainer);
    }
  }
})();
