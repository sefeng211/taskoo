import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';

export const OPERATION_TYPES = { AGENDA : "AGENDA", LIST: "LIST" };

export function createTaskCard(taskBody) {
  const task = document.createElement("div");
  task.classList.add("card");

  const contentContainer = document.createElement("div");
  contentContainer.classList.add("card-content");
  contentContainer.classList.add("p-0");
  contentContainer.classList.add("pl-4");
  contentContainer.classList.add("pr-4");
  task.appendChild(contentContainer);

  const content = document.createElement("div");
  content.classList.add("card-body");
  content.classList.add("is-size-6");

  content.innerHTML = taskBody;

  contentContainer.appendChild(content);

  const tools = document.getElementById("card-tools").content.cloneNode(true);
  contentContainer.appendChild(tools);

  // Add event handlers to those card-tools-button
  contentContainer.querySelector(".card-tools-complete-button").addEventListener("click", function() {
    const endpoint = SERVER_ENDPOINT_MAPPING["state_change"];
    let result = fetch(endpoint, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ "data": "316 @complete" })
    });

  });
  task.appendChild(contentContainer);
  return task;
}

function createContextNavBar(context) {
  const nav = document.createElement("nav");
  nav.classList.add("breadcrumb");
  nav.classList.add("is-centered");
  nav.classList.add("has-bullet-separator");
  nav.classList.add("is-medium");

  const ul = document.createElement("ul");

  context.forEach((c) => {
    const li = document.createElement("li");
    const a = document.createElement("a");
    a.innerHTML = c;
    a.classList.add("pr-0");

    a.addEventListener("click", function() {
      handleContextSwitch(a.innerHTML);
    });

    li.appendChild(a);

    ul.appendChild(li);
  });

  nav.appendChild(ul);
  return nav;
}

function handleContextSwitch(context) {
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
  for (let task of tasksForContext) {
    // Completed tasks are not displayed
    // TODO: Allow users to toggle this
    if (task.state != "completed") {
      tasksContainer.appendChild(createTaskCard(task.body));
    }
  }

  document.querySelector("main").appendChild(tasksContainer);
}

var taskCache = {};

function handleTasks(tasksWithContext) {
  taskCache = {};

  // Get the returned context
  let context = [];
  tasksWithContext.forEach(function(element) {
    const c = element[0];
    const tasksForContext = element[1];
    if (element[0] !== "") {
      context.push(c);
      taskCache[c] = tasksForContext;
    }
  });

  // Add the breadcrumb bar
  const nav = createContextNavBar(context);
  document.querySelector("main").prepend(nav);

  // Switch to the first context by default
  handleContextSwitch(context[0]);
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
