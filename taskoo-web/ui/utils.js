import { SERVER_ENDPOINT_RUN, SERVER_ENDPOINT_TODAY } from './consts.mjs';

customElements.define(
  "my-tasks",
  class extends HTMLElement {
    constructor() {
      super();
      console.log("here");
      let template = document.getElementById("my-tasks");
      let templateContent = template.content;

      const shadowRoot = this.attachShadow({ mode: "open" });
      this.renderTask();
      const result = fetch(SERVER_ENDPOINT_TODAY);
      result.then((data) => data.json())
        .then((data) => {
          const list = templateContent.getElementById("list");
          for (const t of data) {
            var li = document.createElement("li");
            li.appendChild(document.createTextNode(t.body));
            list.appendChild(li);
          }
          console.log("here");
          shadowRoot.appendChild(list);
        });
    }
    renderTask() {
    }
  }
);

function sendGetTaskQuery() {
  const data = document.querySelector("input").value;
  fetch(SERVER_ENDPOINT_RUN, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ "data": data })
  });
  console.log(data);
}
