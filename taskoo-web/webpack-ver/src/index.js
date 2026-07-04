import './style.css';
import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';
import {
  buildBulkModificationCommand,
  navCounts,
  pruneSelectionForVisibleTasks,
  tagView,
} from './ui_logic.mjs';

const BUILTIN_STATES = ['ready', 'started', 'blocked', 'completed'];
const AGENDA_RANGE_KEY = 'taskoo.agendaDays';

const state = {
  view: 'inbox',
  title: 'Inbox',
  subtitle: 'Clarify and organize newly captured tasks',
  groups: [],
  tasks: [],
  navTasks: [],
  navAgendaTasks: [],
  selectedTaskIds: new Set(),
  clientFilter: null,
  metadata: {
    contexts: ['inbox'],
    tags: [],
    states: BUILTIN_STATES,
    custom_states: [],
    priorities: ['H', 'M', 'L'],
  },
  loading: false,
  agendaDays: Number(window.localStorage.getItem(AGENDA_RANGE_KEY) || 10),
};

function endpoint(name) {
  return SERVER_ENDPOINT_MAPPING[name];
}

async function request(name, {method = 'GET', data} = {}) {
  const url = endpoint(name);
  if (!url) {
    throw new Error(`Unknown endpoint: ${name}`);
  }

  const options = {method, headers: {'Content-Type': 'application/json'}};
  if (data !== undefined) {
    options.body = JSON.stringify({data});
  }

  const response = await fetch(url, options);
  const payload = await response.json().catch(() => ({}));
  if (!response.ok || payload.error) {
    throw new Error(payload.error || `Request failed with ${response.status}`);
  }
  return payload;
}

function flattenGroups(groups) {
  return groups.flatMap((group) => group[1] || []);
}

function activeTasks(tasks) {
  return tasks.filter((task) => task.state !== 'completed');
}

function todayIso() {
  return new Date().toISOString().slice(0, 10);
}

function toIsoDate(date) {
  return date.toISOString().slice(0, 10);
}

function addDays(date, days) {
  const nextDate = new Date(date);
  nextDate.setDate(nextDate.getDate() + days);
  return nextDate;
}

function formatDate(value) {
  if (!value) {
    return '';
  }
  return value.split(' ')[0];
}

function isToday(value) {
  return formatDate(value) === todayIso();
}

function isPast(value) {
  const date = formatDate(value);
  return date && date < todayIso();
}

function addClassByState(task) {
  return `task-row state-${task.state || 'ready'}`;
}

function taskDateLabel(task) {
  if (task.date_due) {
    return `Due ${formatDate(task.date_due)}`;
  }
  if (task.date_scheduled) {
    return `Scheduled ${formatDate(task.date_scheduled)}`;
  }
  return '';
}

function setLoading(isLoading) {
  state.loading = isLoading;
  document.body.classList.toggle('is-loading', isLoading);
  const status = document.getElementById('sync-status');
  if (status) {
    status.textContent = isLoading ? 'Syncing' : 'Ready';
  }
}

function toast(message) {
  const stack = document.getElementById('toast-stack');
  const item = document.createElement('div');
  item.className = 'toast';
  item.textContent = message;
  stack.appendChild(item);
  window.setTimeout(() => {
    item.classList.add('is-fading');
    window.setTimeout(() => item.remove(), 180);
  }, 2200);
}

function showError(error) {
  toast(error.message || String(error));
}

function html(strings, ...values) {
  return strings.reduce((result, part, index) => {
    const value = values[index] ?? '';
    return result + part + value;
  }, '');
}

function renderShell() {
  document.body.innerHTML = html`
    <div id="toast-stack" class="toast-stack" aria-live="polite"></div>
    <div class="app-shell">
      <aside class="rail" aria-label="Task navigation">
        <div class="brand">
          <div class="brand-dot"><i class="fas fa-check"></i></div>
          <div>
            <h1>Taskoo</h1>
            <span>GTD workspace</span>
          </div>
        </div>
        <button class="compose-button" type="button" id="compose-button">
          <i class="fas fa-plus"></i>
          <span>Add task</span>
        </button>
        <nav class="nav-list">
          ${navButton('inbox', 'fas fa-inbox', 'Inbox')}
          ${navButton('agenda', 'fas fa-calendar-alt', 'Agenda')}
          ${navButton('all', 'fas fa-layer-group', 'All')}
          ${navButton('started', 'fas fa-play', 'Started')}
          ${navButton('blocked', 'fas fa-ban', 'Blocked')}
          ${navButton('completed', 'fas fa-check-double', 'Completed')}
        </nav>
        <div class="rail-section">
          <div class="rail-heading">Contexts</div>
          <div id="context-list" class="context-list"></div>
        </div>
        <div class="rail-section">
          <div class="rail-heading">Tags</div>
          <div id="tag-list" class="context-list"></div>
        </div>
        <div class="rail-footer">
          <span class="status-dot"></span>
          <span id="sync-status">Ready</span>
        </div>
      </aside>

      <main class="workspace">
        <header class="topbar">
          <div>
            <div class="eyebrow">Taskoo</div>
            <h2 id="view-title">Inbox</h2>
            <p id="view-subtitle">Clarify and organize newly captured tasks</p>
          </div>
          <div class="topbar-actions">
            <button class="icon-button" type="button" id="refresh-button" title="Refresh" aria-label="Refresh">
              <i class="fas fa-sync-alt"></i>
            </button>
          </div>
        </header>

        <section class="quick-add" aria-label="Quick add task">
          <form id="quick-add-form">
            <input id="quick-add-input" autocomplete="off" placeholder="Add a task. Try: Call Alex c:work +phone d:2026-07-05 pri:H">
            <button type="submit" title="Add task" aria-label="Add task"><i class="fas fa-plus"></i></button>
          </form>
        </section>

        <section class="search-strip" aria-label="Task query">
          <label>
            <i class="fas fa-search"></i>
            <input id="query-input" autocomplete="off" placeholder="Search with CLI syntax: c:inbox +tag ^waiting d:2026-07-04">
          </label>
          <button type="button" id="query-button">Search</button>
        </section>

        <section class="summary-grid" aria-label="Task summary">
          <div>
            <span>Active</span>
            <strong id="active-count">0</strong>
          </div>
          <div>
            <span>Started</span>
            <strong id="started-count">0</strong>
          </div>
          <div>
            <span>Blocked</span>
            <strong id="blocked-count">0</strong>
          </div>
          <div>
            <span>Completed</span>
            <strong id="completed-count">0</strong>
          </div>
        </section>

        <section id="agenda-controls" class="agenda-controls" aria-label="Agenda range">
          <div>
            <span>Agenda range</span>
            <strong id="agenda-range-label">10 days</strong>
          </div>
          <div class="agenda-presets">
            <button type="button" data-agenda-days="7">7d</button>
            <button type="button" data-agenda-days="10">10d</button>
            <button type="button" data-agenda-days="30">30d</button>
          </div>
          <label>
            <span>Days</span>
            <input id="agenda-days-input" type="number" min="1" max="366" step="1">
          </label>
        </section>

        <section id="filter-chips" class="filter-chips" aria-label="View filters"></section>
        <section id="bulk-actions" class="bulk-actions" aria-label="Bulk task actions"></section>
        <section id="task-board" class="task-board" aria-label="Tasks"></section>
      </main>
    </div>
  `;

  bindShellEvents();
}

function navButton(view, icon, label) {
  return `<button class="nav-item" type="button" data-view="${view}">
    <i class="${icon}"></i>
    <span>${label}</span>
    <strong data-count-for="${view}">0</strong>
  </button>`;
}

function bindShellEvents() {
  document.querySelectorAll('[data-view]').forEach((button) => {
    button.addEventListener('click', () => loadView(button.dataset.view));
  });
  document.getElementById('compose-button').addEventListener('click', () => {
    document.getElementById('quick-add-input').focus();
  });
  document.getElementById('refresh-button').addEventListener('click', reload);
  document.getElementById('quick-add-form').addEventListener('submit', addTask);
  document.getElementById('query-button').addEventListener('click', runQuery);
  document.getElementById('query-input').addEventListener('keydown', (event) => {
    if (event.key === 'Enter') {
      event.preventDefault();
      runQuery();
    }
  });
  document.querySelectorAll('[data-agenda-days]').forEach((button) => {
    button.addEventListener('click', () => setAgendaDays(Number(button.dataset.agendaDays)));
  });
  document.getElementById('agenda-days-input').addEventListener('change', (event) => {
    setAgendaDays(Number(event.target.value));
  });
}

async function refreshMetadata() {
  state.metadata = await request('metadata');
  state.metadata.states = [...BUILTIN_STATES, ...(state.metadata.custom_states || [])];
  renderContexts();
  renderTags();
}

async function refreshNavCounts() {
  const [allGroups, agendaGroups] = await Promise.all([
    request('list', {method: 'POST', data: ''}),
    request('agenda', {method: 'POST', data: `${agendaRange().start} ${agendaRange().end}`}),
  ]);
  state.navTasks = flattenGroups(Array.isArray(allGroups) ? allGroups : []);
  state.navAgendaTasks = flattenGroups(Array.isArray(agendaGroups) ? agendaGroups : []);
}

function renderContexts() {
  const container = document.getElementById('context-list');
  container.replaceChildren();
  state.metadata.contexts.forEach((context) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'context-item';
    button.textContent = context;
    button.addEventListener('click', () => loadContext(context));
    container.appendChild(button);
  });
}

function renderTags() {
  const container = document.getElementById('tag-list');
  container.replaceChildren();
  state.metadata.tags.forEach((tag) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = 'context-item';
    button.textContent = `#${tag}`;
    button.addEventListener('click', () => loadTag(tag));
    container.appendChild(button);
  });
}

async function addTask(event) {
  event.preventDefault();
  const input = document.getElementById('quick-add-input');
  const value = input.value.trim();
  if (!value) {
    return;
  }
  setLoading(true);
  try {
    await request('add', {method: 'POST', data: value});
    input.value = '';
    await refreshMetadata();
    await reload();
    toast('Task added');
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

async function runQuery() {
  const query = document.getElementById('query-input').value.trim();
  state.view = 'search';
  state.title = query ? 'Search' : 'All tasks';
  state.subtitle = query || 'Every context, grouped by list';
  const stateMatch = query.match(/^@(.+)$/);
  state.clientFilter = stateMatch ? (task) => task.state === stateMatch[1] : null;
  await loadList(query);
}

async function loadContext(context) {
  state.view = `context:${context}`;
  state.title = context;
  state.subtitle = 'Context list';
  state.clientFilter = null;
  document.getElementById('query-input').value = `c:${context}`;
  await loadList(`c:${context}`);
}

async function loadTag(tag) {
  const nextView = tagView(tag);
  state.view = nextView.view;
  state.title = nextView.title;
  state.subtitle = nextView.subtitle;
  state.clientFilter = null;
  document.getElementById('query-input').value = nextView.query;
  await loadList(nextView.query);
}

async function loadView(view) {
  state.view = view;
  state.clientFilter = null;
  document.getElementById('query-input').value = '';

  if (view === 'agenda') {
    await loadAgendaRange();
    return;
  }
  if (view === 'all') {
    state.title = 'All';
    state.subtitle = 'All active and completed tasks';
    await loadList('');
    return;
  }

  const labels = {
    inbox: ['Inbox', 'Clarify and organize newly captured tasks', 'c:Inbox'],
    started: ['Started', 'Tasks currently in motion', ''],
    blocked: ['Blocked', 'Waiting or stuck tasks', ''],
    completed: ['Completed', 'Done tasks for review', ''],
  };
  const [title, subtitle, query] = labels[view] || labels.inbox;
  state.title = title;
  state.subtitle = subtitle;
  if (['started', 'blocked', 'completed'].includes(view)) {
    state.clientFilter = (task) => task.state === view;
  }
  await loadList(query);
}

async function reload() {
  if (state.view === 'agenda') {
    await loadView(state.view);
  } else if (state.view.startsWith('context:')) {
    await loadContext(state.view.slice('context:'.length));
  } else if (state.view.startsWith('tag:')) {
    await loadTag(state.view.slice('tag:'.length));
  } else if (state.view === 'search') {
    await runQuery();
  } else {
    await loadView(state.view);
  }
}

function agendaRange() {
  const start = todayIso();
  const end = toIsoDate(addDays(new Date(), Math.max(state.agendaDays, 1) - 1));
  return {start, end};
}

async function loadAgendaRange() {
  const {start, end} = agendaRange();
  state.title = 'Agenda';
  state.subtitle = `${start} to ${end}`;
  await loadAgenda(`${start} ${end}`);
}

function setAgendaDays(days) {
  const nextDays = Math.min(Math.max(Number.isFinite(days) ? Math.round(days) : 10, 1), 366);
  state.agendaDays = nextDays;
  window.localStorage.setItem(AGENDA_RANGE_KEY, String(nextDays));
  renderAgendaControls();
  if (state.view === 'agenda') {
    loadAgendaRange();
  }
}

async function loadAgenda(query) {
  setLoading(true);
  try {
    await refreshNavCounts();
    const groups = await request('agenda', {method: 'POST', data: query});
    setGroups(groups);
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

async function loadList(query) {
  setLoading(true);
  try {
    await refreshNavCounts();
    const groups = await request('list', {method: 'POST', data: query});
    setGroups(groups);
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

function setGroups(groups) {
  const rawGroups = Array.isArray(groups) ? groups : [];
  state.groups = state.clientFilter
    ? rawGroups.map(([name, tasks]) => [name, (tasks || []).filter(state.clientFilter)])
    : rawGroups;
  state.tasks = flattenGroups(state.groups);
  state.selectedTaskIds = pruneSelectionForVisibleTasks(state.selectedTaskIds, state.tasks);
  render();
}

function render() {
  document.getElementById('view-title').textContent = state.title;
  document.getElementById('view-subtitle').textContent = state.subtitle;
  document.querySelectorAll('[data-view]').forEach((item) => {
    item.classList.toggle('is-active', item.dataset.view === state.view);
  });
  renderSummary();
  renderAgendaControls();
  renderChips();
  renderBulkActions();
  renderTasks();
}

function renderAgendaControls() {
  const controls = document.getElementById('agenda-controls');
  const input = document.getElementById('agenda-days-input');
  const label = document.getElementById('agenda-range-label');
  if (!controls || !input || !label) {
    return;
  }
  controls.classList.toggle('is-active', state.view === 'agenda');
  input.value = state.agendaDays;
  label.textContent = `${state.agendaDays} ${state.agendaDays === 1 ? 'day' : 'days'}`;
  document.querySelectorAll('[data-agenda-days]').forEach((button) => {
    button.classList.toggle('is-active', Number(button.dataset.agendaDays) === state.agendaDays);
  });
}

function renderSummary() {
  const all = state.tasks;
  const navAll = state.navTasks;
  document.getElementById('active-count').textContent = activeTasks(all).length;
  document.getElementById('started-count').textContent = all.filter((task) => task.state === 'started').length;
  document.getElementById('blocked-count').textContent = all.filter((task) => task.state === 'blocked').length;
  document.getElementById('completed-count').textContent = all.filter((task) => task.state === 'completed').length;

  const counts = navCounts(navAll, state.navAgendaTasks);
  Object.entries(counts).forEach(([key, count]) => {
    const node = document.querySelector(`[data-count-for="${key}"]`);
    if (node) {
      node.textContent = count;
    }
  });
}

function renderChips() {
  const container = document.getElementById('filter-chips');
  container.replaceChildren();
  const contexts = [...new Set(state.tasks.map((task) => task.context).filter(Boolean))];
  const tags = [...new Set(state.tasks.flatMap((task) => task.tags || []))];
  [
    ...contexts.map((value) => ({label: value, query: `c:${value}`, icon: 'fas fa-list'})),
    ...tags.map((value) => ({label: `#${value}`, query: `+${value}`, icon: 'fas fa-tag'})),
  ].slice(0, 12).forEach((chip) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.innerHTML = `<i class="${chip.icon}"></i><span>${chip.label}</span>`;
    button.addEventListener('click', async () => {
      document.getElementById('query-input').value = chip.query;
      state.view = 'search';
      state.title = chip.label;
      state.subtitle = chip.query;
      await loadList(chip.query);
    });
    container.appendChild(button);
  });
}

function selectedBulkTasks() {
  return state.tasks.filter((task) => state.selectedTaskIds.has(task.id));
}

function toggleBulkSelection(task, checked) {
  if (checked) {
    state.selectedTaskIds.add(task.id);
  } else {
    state.selectedTaskIds.delete(task.id);
  }
  render();
}

function clearBulkSelection() {
  state.selectedTaskIds.clear();
  render();
}

function selectAllVisibleTasks() {
  state.tasks.forEach((task) => state.selectedTaskIds.add(task.id));
  render();
}

function renderBulkActions() {
  const container = document.getElementById('bulk-actions');
  const selected = selectedBulkTasks();
  const count = selected.length;
  const singleTask = count === 1 ? selected[0] : null;

  container.classList.toggle('is-active', count > 0);
  if (count === 0) {
    container.replaceChildren();
    return;
  }

  container.innerHTML = `
    <div class="bulk-summary">
      <strong>${count}</strong>
      <span>${count === 1 ? 'task selected' : 'tasks selected'}</span>
    </div>
    <button class="secondary-button" type="button" id="bulk-select-all">
      <i class="fas fa-check-square"></i><span>Select all</span>
    </button>
    <button class="secondary-button" type="button" id="bulk-clear">
      <i class="fas fa-times"></i><span>Clear</span>
    </button>
    <form id="bulk-form" class="bulk-form">
      <select name="state" aria-label="State">
        <option value="">State</option>
        ${state.metadata.states.map((item) => `<option value="${item}">${item}</option>`).join('')}
      </select>
      <select name="priority" aria-label="Priority">
        <option value="">Priority</option>
        ${state.metadata.priorities.map((item) => `<option value="${item}">${item}</option>`).join('')}
      </select>
      <input name="context" type="text" placeholder="Context" aria-label="Context">
      <input name="tags" type="text" placeholder="Add tags" aria-label="Add tags">
      <input name="remove_tags" type="text" placeholder="Remove tags" aria-label="Remove tags">
      <input name="due" type="date" aria-label="Due date">
      <input name="scheduled" type="date" aria-label="Scheduled date">
      <input name="due_repeat" type="text" placeholder="Due repeat" aria-label="Due repeat" value="${singleTask ? singleTask.repetition_due || '' : ''}">
      <input name="scheduled_repeat" type="text" placeholder="Schedule repeat" aria-label="Schedule repeat" value="${singleTask ? singleTask.repetition_scheduled || '' : ''}">
      <button class="primary-button" type="submit">
        <i class="fas fa-magic"></i><span>Apply</span>
      </button>
    </form>
    <button class="danger-button" type="button" id="bulk-delete">
      <i class="fas fa-trash"></i><span>Delete</span>
    </button>
  `;

  document.getElementById('bulk-select-all').addEventListener('click', selectAllVisibleTasks);
  document.getElementById('bulk-clear').addEventListener('click', clearBulkSelection);
  document.getElementById('bulk-delete').addEventListener('click', deleteSelectedTasks);
  document.getElementById('bulk-form').addEventListener('submit', modifySelectedTasks);
}

function renderTasks() {
  const board = document.getElementById('task-board');
  board.replaceChildren();

  const visibleGroups = state.groups
    .map(([name, tasks]) => [name, tasks || []])
    .filter(([, tasks]) => tasks.length > 0);

  if (visibleGroups.length === 0) {
    board.innerHTML = `<div class="empty-state">
      <i class="fas fa-check-circle"></i>
      <h3>Nothing here</h3>
      <p>Capture a task or change the current view.</p>
    </div>`;
    return;
  }

  visibleGroups.forEach(([name, tasks]) => {
    const section = document.createElement('section');
    section.className = 'task-section';
    section.innerHTML = `<header><h3>${name || 'Tasks'}</h3><span>${tasks.length}</span></header>`;
    const rows = document.createElement('div');
    rows.className = 'task-rows';
    tasks.forEach((task) => rows.appendChild(createTaskRow(task)));
    section.appendChild(rows);
    board.appendChild(section);
  });
}

function createTaskRow(task) {
  const row = document.createElement('article');
  row.className = addClassByState(task);
  row.classList.toggle('is-bulk-selected', state.selectedTaskIds.has(task.id));
  row.addEventListener('click', () => {
    toggleBulkSelection(task, !state.selectedTaskIds.has(task.id));
  });

  const bulkCheck = document.createElement('input');
  bulkCheck.type = 'checkbox';
  bulkCheck.className = 'bulk-check';
  bulkCheck.checked = state.selectedTaskIds.has(task.id);
  bulkCheck.title = `Select task #${task.id}`;
  bulkCheck.setAttribute('aria-label', `Select task #${task.id}`);
  bulkCheck.addEventListener('click', (event) => event.stopPropagation());
  bulkCheck.addEventListener('change', (event) => toggleBulkSelection(task, event.target.checked));

  const check = document.createElement('button');
  check.type = 'button';
  check.className = 'task-check';
  check.title = task.state === 'completed' ? 'Mark ready' : 'Mark completed';
  check.innerHTML = `<i class="fas ${task.state === 'completed' ? 'fa-check' : 'fa-circle'}"></i>`;
  check.addEventListener('click', async (event) => {
    event.stopPropagation();
    await setTaskState(task, task.state === 'completed' ? 'ready' : 'completed');
  });

  const meta = [
    task.priority ? `<span class="badge priority">P${task.priority}</span>` : '',
    task.context ? `<span class="badge">${task.context}</span>` : '',
    taskDateLabel(task) ? `<span class="badge ${isPast(task.date_due || task.date_scheduled) ? 'danger' : ''}">${taskDateLabel(task)}</span>` : '',
    ...(task.tags || []).map((tag) => `<span class="badge tag">#${tag}</span>`),
  ].join('');

  row.innerHTML = `
    <div class="task-main">
      <div class="task-title-line">
        <strong>${task.body}</strong>
        <span>#${task.id}</span>
      </div>
      <div class="task-meta">${meta}</div>
    </div>
    <div class="task-state">${task.state || 'ready'}</div>
  `;
  row.prepend(check);
  row.prepend(bulkCheck);
  return row;
}

async function setTaskState(task, nextState) {
  setLoading(true);
  try {
    await request('modify', {method: 'POST', data: `${task.id} @${nextState}`});
    await reload();
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

function selectedTaskIdsText() {
  return selectedBulkTasks().map((task) => task.id).join(' ');
}

function bulkModificationTokens(form) {
  const data = new FormData(form);
  return buildBulkModificationCommand(selectedBulkTasks().map((task) => task.id), {
    context: data.get('context'),
    tags: data.get('tags'),
    remove_tags: data.get('remove_tags'),
    state: data.get('state'),
    priority: data.get('priority'),
    due: data.get('due'),
    due_repeat: data.get('due_repeat'),
    scheduled: data.get('scheduled'),
    scheduled_repeat: data.get('scheduled_repeat'),
  });
}

async function modifySelectedTasks(event) {
  event.preventDefault();
  const command = bulkModificationTokens(event.currentTarget);
  if (state.selectedTaskIds.size === 0 || command === selectedTaskIdsText()) {
    toast('Choose tasks and at least one change');
    return;
  }

  setLoading(true);
  try {
    await request('modify', {method: 'POST', data: command});
    state.selectedTaskIds.clear();
    await refreshMetadata();
    await reload();
    toast('Tasks updated');
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

async function deleteSelectedTasks() {
  const selected = selectedBulkTasks();
  if (selected.length === 0) {
    return;
  }
  if (!window.confirm(`Delete ${selected.length} ${selected.length === 1 ? 'task' : 'tasks'}?`)) {
    return;
  }

  setLoading(true);
  try {
    await request('delete', {method: 'POST', data: selected.map((task) => task.id).join(' ')});
    state.selectedTaskIds.clear();
    await refreshMetadata();
    await reload();
    toast('Tasks deleted');
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

async function boot() {
  renderShell();
  setLoading(true);
  try {
    await refreshMetadata();
    await loadView('inbox');
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

window.addEventListener('load', boot);
