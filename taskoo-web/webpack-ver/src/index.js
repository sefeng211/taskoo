import _ from 'lodash';
import './style.css';
import {handleOperation, createTaskCard, OPERATION_TYPES} from './task_helpers.js';
import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';

function resetTabItems() {
  const tabs = document.querySelector(".tabs");
  const tab_items = tabs.querySelectorAll("li");
  for (const tab_item of tab_items) {
    tab_item.classList.remove("is-active");
  }
}

// Initialize tabs
function InitTabs() {
  const tabs = document.querySelector(".tabs");
  const tab_items = tabs.querySelectorAll("li");
  for (const tab_item of tab_items) {
    tab_item.addEventListener("click", function() {
      const classList = tab_item.classList;

      SwtichTab(tab_item);
    });
  }
}

function SwtichTab(newTab) {
  window.location.hash = `#${newTab.id}`;

  resetTabItems();
  if (!newTab.classList.contains("is-active")) {
    newTab.classList.add("is-active");
  }

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
        handleOperation(OPERATION_TYPES.AGENDA, r);
      });
    });
  }
}

function Init() {
  const removeErrorButton = document.getElementById("remove-error-message");
  removeErrorButton.addEventListener("click", function() {
    const notification = document.getElementById('error-notification');
    notification.style.display = 'none';
  });
}

window.onload = function() {
  InitTabs();
  Init();

  const currentHash = window.location.hash.replace('#', '');
  console.log(currentHash);
  if (currentHash === "today-tab" || !currentHash) {
    console.log("switch to today tab");
    SwtichTab(document.getElementById("today-tab"));
  } else {
    console.log("switch to inbox tab");
    SwtichTab(document.getElementById("inbox-tab"));
  }


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

  const searchInput = document.getElementById("search-input");
  searchInput.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      event.preventDefault(); // Prevent the default form submission behavior
      console.log('Enter key pressed, perform search');
      resetTabItems();

      const body = searchInput.value;
      const endpoint = SERVER_ENDPOINT_MAPPING["list"];
      let result = fetch(endpoint, {
        method: "POST",
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ "data": searchInput.value })
      });

      result.then(function(data) {
        data.json().then(function(r) {
          handleOperation(OPERATION_TYPES.LIST, r);
        });
      });

      // TODO: should the result of the operation
    }
  });
}
