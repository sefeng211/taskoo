import {
  SERVER_ENDPOINT_RUN,
  SERVER_ENDPOINT_TODAY,
  SERVER_ENDPOINT_MAPPING} from './consts.mjs';

import {TaskDisplay} from './task_display.mjs';

function displayError(message) {
  const errorBox = document.querySelector(".error-message");
  errorBox.textContent = message;
}

async function RenderTasks(tasks) {
  // Remove existing tasks
}

// The format of the returned data from each operation
// may be different, hence a different handling function
// is required.
function handleAgenda(data) {
  for (const day of data) {
    const taskContainer = document.createElement("my-tasks");
    let listGroup = createListGroupForTasks(day[1]);
    taskContainer.appendChild(listGroup);
    document.querySelector("main").appendChild(taskContainer);
  }
}

function createListGroupForTasks(tasks) {
  const container = document.createElement("div");
  container.classList.add("list-group");

  const taskDisplay = new TaskDisplay();

  for (const task of tasks) {
    container.appendChild(taskDisplay.createTask(task));
  }
  return container;
}

customElements.define(
  "my-tasks",
  class extends HTMLElement {
    constructor() {
      super();
    }
  }
);

async function runTask(method) {
  console.log("runTask");
  const options = document.querySelector(".search-dropdown");
  const endpoint = SERVER_ENDPOINT_MAPPING[method];

  const data = document.querySelector("input").value;
  console.log(`sendGetTaskQuery data="${data}" endpoint=${endpoint}`);
  let result = await fetch(endpoint, {
    method: "POST",
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ "data": data })
  });

  result.json().then(data => {
    console.log(data);
    if (data.error) {
      displayError(data.error);
    } else {
      const existingTaskContainers = document.querySelectorAll("my-tasks");
      for (const container of existingTaskContainers) {
        container.remove();
      }

      if (method === "agenda") {
        handleAgenda(data);
        return;
      }

      for (const contextAndTaskTuple of data) {
        const tasks = JSON.parse(contextAndTaskTuple[1]);
        if (!tasks.length) {
          continue;
        }

        const taskContainer = document.createElement("my-tasks");

        // display the context
        const context = document.createElement("span");
        context.innerHTML = contextAndTaskTuple[0];

        taskContainer.appendChild(context);
        taskContainer.appendChild(createListGroupForTasks(tasks));
        document.querySelector("main").appendChild(taskContainer);
      }
    }
  });
}

document.querySelector(".get-button").addEventListener("click", function() {
  runTask("list");
});

document.querySelector(".add-button").addEventListener("click", function() {
  runTask("add");
});

document.querySelector(".agenda-button").addEventListener("click", function() {
  runTask("agenda");
});

