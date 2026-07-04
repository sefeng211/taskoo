import './style.css';
import {SERVER_ENDPOINT_MAPPING} from './consts.mjs';

const BUILTIN_STATES = ['ready', 'started', 'blocked', 'completed'];
const AGENDA_RANGE_KEY = 'taskoo.agendaDays';

const state = {
  view: 'inbox',
  title: 'Inbox',
  subtitle: 'Clarify and organize newly captured tasks',
  groups: [],
  tasks: [],
  clientFilter: null,
  selectedTaskId: null,
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

function isCompactLayout() {
  return window.matchMedia('(max-width: 1120px)').matches;
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

function escapeToken(value) {
  return (value || '').trim().replace(/\s+/g, '-');
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
        <section id="task-board" class="task-board" aria-label="Tasks"></section>
      </main>

      <aside id="detail-panel" class="detail-panel" aria-label="Task details"></aside>
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
  if (!state.tasks.some((task) => task.id === state.selectedTaskId)) {
    state.selectedTaskId = isCompactLayout() ? null : state.tasks[0]?.id ?? null;
  }
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
  renderTasks();
  renderDetail();
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
  document.getElementById('active-count').textContent = activeTasks(all).length;
  document.getElementById('started-count').textContent = all.filter((task) => task.state === 'started').length;
  document.getElementById('blocked-count').textContent = all.filter((task) => task.state === 'blocked').length;
  document.getElementById('completed-count').textContent = all.filter((task) => task.state === 'completed').length;

  const counts = {
    inbox: all.filter((task) => task.context === 'inbox' && task.state !== 'completed').length,
    agenda: all.length,
    all: all.length,
    started: all.filter((task) => task.state === 'started').length,
    blocked: all.filter((task) => task.state === 'blocked').length,
    completed: all.filter((task) => task.state === 'completed').length,
  };
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
  row.classList.toggle('is-selected', task.id === state.selectedTaskId);
  row.addEventListener('click', () => {
    state.selectedTaskId = task.id;
    render();
  });

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
  return row;
}

function selectedTask() {
  return state.tasks.find((task) => task.id === state.selectedTaskId) || null;
}

function renderDetail() {
  const panel = document.getElementById('detail-panel');
  const task = selectedTask();
  panel.classList.toggle('has-task', Boolean(task));
  if (!task) {
    panel.innerHTML = `<div class="detail-empty"><i class="fas fa-tasks"></i><p>Select a task</p></div>`;
    return;
  }

  panel.innerHTML = `
    <div class="detail-header">
      <button class="icon-button" type="button" id="close-detail" title="Clear selection" aria-label="Clear selection">
        <i class="fas fa-times"></i>
      </button>
      <span>#${task.id}</span>
    </div>
    <h2>${task.body}</h2>
    <div class="detail-actions">
      ${stateButton(task, 'ready', 'fa-undo')}
      ${stateButton(task, 'started', 'fa-play')}
      ${stateButton(task, 'blocked', 'fa-ban')}
      ${stateButton(task, 'completed', 'fa-check')}
    </div>
    <form id="detail-form" class="detail-form">
      ${inputField('Context', 'context', task.context, 'text', 'inbox')}
      ${inputField('Tags', 'tags', (task.tags || []).join(', '), 'text', 'next, waiting')}
      ${inputField('Remove tags', 'remove_tags', '', 'text', 'oldtag, someday')}
      ${selectField('State', 'state', task.state, state.metadata.states)}
      ${selectField('Priority', 'priority', task.priority, ['', ...state.metadata.priorities])}
      ${inputField('Due date', 'due', formatDate(task.date_due), 'date')}
      ${inputField('Scheduled', 'scheduled', formatDate(task.date_scheduled), 'date')}
      ${inputField('Due repeat', 'due_repeat', task.repetition_due, 'text', 'weekly')}
      ${inputField('Schedule repeat', 'scheduled_repeat', task.repetition_scheduled, 'text', 'daily')}
      <button class="primary-button" type="submit">Save changes</button>
    </form>
    ${task.annotation ? `<section class="annotation"><h3>Annotation</h3><p>${task.annotation}</p></section>` : ''}
    <button class="danger-button" type="button" id="delete-task"><i class="fas fa-trash"></i><span>Delete task</span></button>
  `;

  document.getElementById('close-detail').addEventListener('click', () => {
    state.selectedTaskId = null;
    renderDetail();
  });
  document.querySelectorAll('[data-next-state]').forEach((button) => {
    button.addEventListener('click', async () => setTaskState(task, button.dataset.nextState));
  });
  document.getElementById('detail-form').addEventListener('submit', (event) => saveTask(event, task));
  document.getElementById('delete-task').addEventListener('click', () => deleteTask(task));
}

function stateButton(task, nextState, icon) {
  return `<button class="${task.state === nextState ? 'is-active' : ''}" type="button" data-next-state="${nextState}">
    <i class="fas ${icon}"></i><span>${nextState}</span>
  </button>`;
}

function inputField(label, name, value, type = 'text', placeholder = '') {
  return `<label><span>${label}</span><input name="${name}" type="${type}" value="${value || ''}" placeholder="${placeholder}"></label>`;
}

function selectField(label, name, value, options) {
  return `<label><span>${label}</span><select name="${name}">
    ${options.map((option) => `<option value="${option}" ${option === value ? 'selected' : ''}>${option || 'none'}</option>`).join('')}
  </select></label>`;
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

function tagTokens(value, prefix) {
  return value
    .split(',')
    .map((tag) => escapeToken(tag))
    .filter(Boolean)
    .map((tag) => `${prefix}${tag}`);
}

async function saveTask(event, task) {
  event.preventDefault();
  const data = new FormData(event.currentTarget);
  const tokens = [`${task.id}`];
  const context = escapeToken(data.get('context'));
  if (context) tokens.push(`c:${context}`);
  tokens.push(...tagTokens(data.get('tags') || '', '+'));
  tokens.push(...tagTokens(data.get('remove_tags') || '', '~'));
  if (data.get('state')) tokens.push(`@${data.get('state')}`);
  if (data.get('priority')) tokens.push(`pri:${data.get('priority')}`);
  if (data.get('due')) tokens.push(`d:${data.get('due')}${data.get('due_repeat') ? `+${escapeToken(data.get('due_repeat'))}` : ''}`);
  if (data.get('scheduled')) tokens.push(`s:${data.get('scheduled')}${data.get('scheduled_repeat') ? `+${escapeToken(data.get('scheduled_repeat'))}` : ''}`);

  setLoading(true);
  try {
    await request('modify', {method: 'POST', data: tokens.join(' ')});
    await refreshMetadata();
    await reload();
    toast('Task updated');
  } catch (error) {
    showError(error);
  } finally {
    setLoading(false);
  }
}

async function deleteTask(task) {
  if (!window.confirm(`Delete "${task.body}"?`)) {
    return;
  }
  setLoading(true);
  try {
    await request('delete', {method: 'POST', data: `${task.id}`});
    state.selectedTaskId = null;
    await refreshMetadata();
    await reload();
    toast('Task deleted');
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
