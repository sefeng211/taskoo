import {
  SERVER_ENDPOINT_RUN,
  SERVER_ENDPOINT_TODAY,
  SERVER_ENDPOINT_MAPPING} from './consts.mjs';

customElements.define(
  "my-tasks",
  class extends HTMLElement {
    constructor() {
      super();
      this.attachShadow({ mode: "open" });
      this.renderTask();
      const result = fetch(SERVER_ENDPOINT_TODAY);
      result.then((data) => data.json())
        .then((data) => {
          const list = document.createElement("list");
          for (const t of data) {
            var li = document.createElement("li");
            li.appendChild(document.createTextNode(t.body));
            list.appendChild(li);
          }
          this.shadowRoot.appendChild(list);
        });
    }
    renderTask() {
    }
  }
);

function sendGetTaskQuery() {
  const options = document.querySelector(".search-dropdown");
  const selected = options[options.selectedIndex].text;
  const endpoint = SERVER_ENDPOINT_MAPPING[selected];

  const data = document.querySelector("input").value;
  console.log(`sendGetTaskQuery data=${data}`);
  fetch(endpoint, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ "data": data })
  });
}

document.querySelector(".search-button").addEventListener("click", sendGetTaskQuery);
