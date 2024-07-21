import _ from 'lodash';
import './style.css';
import {handleOperation, createTaskCard, OPERATION_TYPES} from './task_helpers.js';
import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';


// Initialize tabs
function InitTabs() {
  const tabs = document.querySelector(".tabs");
  const tab_items = tabs.querySelectorAll("li");
  for (const tab_item of tab_items) {
    tab_item.addEventListener("click", function() {
      for (const tab_item of tab_items) {
        tab_item.classList.remove("is-active");
      }
      const classList = tab_item.classList;
      if (!classList.contains("is-active")) {
        classList.add("is-active");
      }

      SwtichTab(tab_item);
    });
  }
}

function SwtichTab(newTab) {
  document.querySelector("main").replaceChildren();

  if (newTab.classList.contains("tab-today")) {
    const endpoint = SERVER_ENDPOINT_MAPPING["agenda"];

    let result = fetch(endpoint, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ "data": "today" })
    });

    result.then(function(data) {
      data.json().then(function(r) {
        handleOperation(OPERATION_TYPES.AGENDA, r);
      });
    });
  } else if (newTab.classList.contains("tab-inbox")) {
    const endpoint = SERVER_ENDPOINT_MAPPING["list"];
    let result = fetch(endpoint, {
      method: "POST",
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ "data": "c:Inbox" })
    });

    result.then(function(data) {
      data.json().then(function(r) {
        const inbox_data = [r[0][0], JSON.parse(r[0][1])];
        const parsed_data = [inbox_data];
        console.log(parsed_data);
        handleOperation(OPERATION_TYPES.AGENDA, parsed_data);
      });
    });
  }
}

window.onload = function() {
  InitTabs();
  SwtichTab(document.querySelector(".tab-today"));

  // Give functionalities to each button
  document.querySelector("#add-button").addEventListener("click", function() {
    const dialog = document.querySelector("#add-modal-template");
    document.body.appendChild(dialog.content);
    document.body.querySelector("#add-dialog").showModal();

    document.querySelector("#submit-button").addEventListener("click", function() {
      const body = document.querySelector("#input-body").value;
      if (body) {
        console.log(body);
        const endpoint = SERVER_ENDPOINT_MAPPING["add"];
        let result = fetch(endpoint, {
          method: "POST",
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ "data": body })
        });

        document.body.querySelector("#add-dialog").close();
        // TODO: should the result of the operation
      } else {
        console.error("Empty input body, doing nothing");
      }
    })
  });
}
