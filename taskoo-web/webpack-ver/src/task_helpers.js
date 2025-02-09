import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';

export const OPERATION_TYPES = { AGENDA : "AGENDA", LIST: "LIST" };

function showError(message) {
    const notification = document.getElementById('error-notification');
    const errorMessage = document.getElementById('error-message');
    errorMessage.textContent = message;
    notification.classList.remove("is-hidden");
}

function resetAllContextSwitches() {
  const container = document.getElementById("context-switches");
  const contextSwitches = container.querySelectorAll("li");
  for (const context of contextSwitches) {
    context.classList.remove("is-active");
  }
}
function createContextNavBar(context) {
  const nav = document.createElement("nav");
  nav.classList.add("breadcrumb");
  nav.classList.add("is-centered");
  nav.classList.add("is-medium");
  nav.classList.add("is-toggle");
  nav.classList.add("is-large");

  const ul = document.createElement("ul");

  context.forEach((c) => {
    const li = document.createElement("li");
    const a = document.createElement("a");
    a.innerHTML = `
        <span class="icon is-small">
          <i class="fas fa-book" aria-hidden="true"></i>
        </span>
        <span>${c}</span>
    `;
    a.classList.add("pr-10");

    a.addEventListener("click", function() {
      resetAllContextSwitches();
      handleContextSwitch(li, c);
    });

    li.appendChild(a);

    ul.appendChild(li);
  });

  nav.appendChild(ul);
  return nav;
}

function handleContextSwitch(listItem, context) {
  if (listItem) {
    listItem.classList.add("is-active");
  }
  // Clear out existing tasks from the previous context
  let tasksContainer = document.getElementById("tasks");
  if (tasksContainer) {
    tasksContainer.replaceChildren();
  } else {
    tasksContainer = document.createElement("div");
    tasksContainer.id = "tasks";
  }

  // let tasksForContext = JSON.parse(taskCache[context]);
  let tasksForContext = taskCache[context];
  console.log(tasksForContext);
  const taskTable = document.getElementById("task-table").getElementsByTagName('tbody')[0];
  taskTable.replaceChildren();
  for (let task of tasksForContext) {
    // Completed tasks are not displayed
    // TODO: Allow users to toggle this
    if (task.state != "completed") {
      const row = taskTable.insertRow();
      const idCell = row.insertCell(0);
      const bodyCell = row.insertCell(1);
      const controlCell = row.insertCell(2);

      function createTags(tags) {
        let ret = '';
        for (const tag of tags) {
          const tagHTML = `<span class="tag is-info is-medium">${tag}</span> `;
          ret += tagHTML;
        }
        return ret;
      }

      idCell.textContent = task.id;
      bodyCell.innerHTML = `
        <div>${task.body}</div>
        <span class="tag is-warning is-medium">${task.state}</span>
        ${createTags(task.tags)}
      `;

      if (task.date_scheduled) {
        bodyCell.innerHTML += `<span class="tag is-warning is-medium">sch: ${task.date_scheduled}</span> `;
      }

      if (task.date_due) {
        bodyCell.innerHTML += `<span class="tag is-warning is-medium">due: ${task.date_due}</span>`;
      }

      if (task.repetition_due) {
        bodyCell.innerHTML += `<span class="tag is-warning is-medium">rep_due: ${task.repetition_due}</span>`;
      }

      if (task.repetition_scheduled) {
        bodyCell.innerHTML += `<span class="tag is-warning is-medium">rep_due: ${task.repetition_scheduled}</span>`;
      }

      // Controls
      controlCell.innerHTML = `
        <span class="icon is-small control-button" id="info-button">
          <i class="fas fa-book" aria-hidden="true"></i>
        </span>
        <span class="icon is-small control-button" id="check-button">
          <i class="fas fa-check" aria-hidden="true"></i>
        </span>
        <span class="icon is-small control-button" id="delete-button">
          <i class="fas fa-trash" aria-hidden="true"></i>
        </span>
      `;

      const deleteButton = controlCell.querySelector("#delete-button");
      deleteButton.addEventListener("click", function() {
        const endpoint = SERVER_ENDPOINT_MAPPING["del"];
        let result = fetch(endpoint, {
          method: "POST",
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ "data": task.id })
        });

        row.remove();
      });
    }
  }
}

var taskCache = {};

function handleTasks(tasksWithContext) {
  console.log("handleTasks");
  taskCache = {};

  // Get the returned context
  let context = [];
  tasksWithContext.forEach(function(element) {
    const c = element[0];
    const tasksForContext = element[1];
    if (element[0] !== "" && tasksForContext.length) {
      context.push(c);
      taskCache[c] = tasksForContext;
    }
  });

  // Add the breadcrumb bar
  const nav = createContextNavBar(context);
  const contextSwitches = document.getElementById("context-switches");
  contextSwitches.replaceChildren();
  contextSwitches.appendChild(nav);

  // Switch to the first context by default
  handleContextSwitch(undefined, context[0]);
}

/*
 * Agenda format looks like
 * [["2022-10-10", [task1, task2]], ["2022-10-10": [task2, task3]]]
 *
 * List format looks like
 * [["Inbox": [task1, task2]], ["Gtd", [task1, task2]]]
 */
export function handleOperation(operation, tasks) {
  handleTasks(tasks);
}
