import './style.css';
import {handleOperation, OPERATION_TYPES} from './task_helpers.js';
import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';

const state = {
  currentView: 'today',
  currentQuery: '',
  loading: false,
};

function endpoint(name) {
  return SERVER_ENDPOINT_MAPPING[name];
}

async function post(name, data) {
  const response = await fetch(endpoint(name), {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({data}),
  });

  if (!response.ok) {
    throw new Error(`Request failed with ${response.status}`);
  }

  return response.json();
}

function showError(message) {
  const notification = document.getElementById('error-notification');
  const errorMessage = document.getElementById('error-message');
  errorMessage.textContent = message;
  notification.classList.remove('is-hidden');
}

function showToast(message) {
  const container = document.getElementById('toast-stack');
  if (!container) {
    return;
  }

  const toast = document.createElement('div');
  toast.className = 'toast';
  toast.textContent = message;
  container.appendChild(toast);

  window.setTimeout(() => {
    toast.classList.add('is-fading');
    window.setTimeout(() => {
      toast.remove();
    }, 220);
  }, 2200);
}

function setLoading(isLoading) {
  state.loading = isLoading;
  document.body.classList.toggle('is-loading-data', isLoading);

  const status = document.getElementById('view-status');
  if (status) {
    status.textContent = isLoading ? 'Syncing' : 'Ready';
  }
}

function setActiveNav(view) {
  document.querySelectorAll('[data-view]').forEach((item) => {
    item.classList.toggle('is-active', item.dataset.view === view);
  });
}

function setViewTitle(title, subtitle) {
  document.getElementById('view-title').textContent = title;
  document.getElementById('view-subtitle').textContent = subtitle;
}

async function renderTaskResponse(operation, tasks, meta) {
  handleOperation(operation, tasks, {
    ...meta,
    onDelete: deleteTask,
    onStateChange: changeTaskState,
    onReload: reloadCurrentView,
  });
}

async function loadToday() {
  state.currentView = 'today';
  state.currentQuery = '';
  window.location.hash = '#today';
  setActiveNav('today');
  setViewTitle('Today', 'Scheduled and due tasks');
  setLoading(true);

  try {
    const tasks = await post('agenda', 'today');
    await renderTaskResponse(OPERATION_TYPES.AGENDA, tasks, {viewName: 'Today'});
  } catch (error) {
    showError(error.message);
  } finally {
    setLoading(false);
  }
}

async function loadInbox() {
  state.currentView = 'inbox';
  state.currentQuery = 'c:Inbox';
  window.location.hash = '#inbox';
  setActiveNav('inbox');
  setViewTitle('Inbox', 'Unsorted tasks');
  setLoading(true);

  try {
    const tasks = await post('list', 'c:Inbox');
    await renderTaskResponse(OPERATION_TYPES.LIST, tasks, {viewName: 'Inbox'});
  } catch (error) {
    showError(error.message);
  } finally {
    setLoading(false);
  }
}

async function searchTasks(query) {
  const trimmedQuery = query.trim();
  if (!trimmedQuery) {
    return loadInbox();
  }

  state.currentView = 'search';
  state.currentQuery = trimmedQuery;
  window.location.hash = '#search';
  setActiveNav('search');
  setViewTitle('Search', trimmedQuery);
  setLoading(true);

  try {
    const tasks = await post('list', trimmedQuery);
    await renderTaskResponse(OPERATION_TYPES.LIST, tasks, {viewName: 'Search'});
  } catch (error) {
    showError(error.message);
  } finally {
    setLoading(false);
  }
}

async function reloadCurrentView() {
  if (state.currentView === 'today') {
    return loadToday();
  }
  if (state.currentView === 'search') {
    return searchTasks(state.currentQuery);
  }
  return loadInbox();
}

async function addTask(body) {
  const trimmedBody = body.trim();
  if (!trimmedBody) {
    return false;
  }

  setLoading(true);
  try {
    await fetch(endpoint('add'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({data: trimmedBody}),
    });
    showToast('Task added');
    return true;
  } catch (error) {
    showError(error.message);
    return false;
  } finally {
    setLoading(false);
  }
}

async function deleteTask(task) {
  setLoading(true);
  try {
    await fetch(endpoint('del'), {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({data: task.id}),
    });
    showToast('Task removed');
    await reloadCurrentView();
  } catch (error) {
    showError(error.message);
  } finally {
    setLoading(false);
  }
}

async function changeTaskState(task, nextState) {
  setLoading(true);
  try {
    await post('state_change', `${task.id} @${nextState}`);
    await reloadCurrentView();
  } catch (error) {
    showError(error.message);
  } finally {
    setLoading(false);
  }
}

function openAddDialog() {
  const dialog = document.getElementById('add-dialog');
  const input = document.getElementById('input-body');
  input.value = '';
  dialog.showModal();
  input.focus();
}

function initAddDialog() {
  const dialog = document.getElementById('add-dialog');
  const form = document.getElementById('add-form');

  document.getElementById('add-button').addEventListener('click', openAddDialog);
  document.getElementById('empty-add-button').addEventListener('click', openAddDialog);
  document.getElementById('cancel-add-button').addEventListener('click', () => dialog.close());

  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    const input = document.getElementById('input-body');
    const created = await addTask(input.value);
    if (created) {
      dialog.close();
      await loadInbox();
    }
  });
}

function initSearch() {
  const searchInput = document.getElementById('search-input');
  const searchButton = document.getElementById('search-button');

  searchButton.addEventListener('click', () => searchTasks(searchInput.value));
  searchInput.addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      event.preventDefault();
      searchTasks(searchInput.value);
    }
  });
}

function initNavigation() {
  document.querySelector('[data-view="today"]').addEventListener('click', loadToday);
  document.querySelector('[data-view="inbox"]').addEventListener('click', loadInbox);
  document.querySelector('[data-view="search"]').addEventListener('click', () => {
    searchTasks(document.getElementById('search-input').value);
  });
}

function initErrorNotification() {
  document.getElementById('remove-error-message').addEventListener('click', () => {
    document.getElementById('error-notification').classList.add('is-hidden');
  });
}

window.onload = function() {
  initErrorNotification();
  initNavigation();
  initSearch();
  initAddDialog();

  const currentHash = window.location.hash.replace('#', '');
  if (currentHash === 'inbox') {
    loadInbox();
  } else {
    loadToday();
  }
};
