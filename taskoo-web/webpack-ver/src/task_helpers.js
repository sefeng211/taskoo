export const OPERATION_TYPES = {AGENDA: 'AGENDA', LIST: 'LIST'};

function clearElement(element) {
  element.replaceChildren();
}

function formatDateTime(value) {
  if (!value) {
    return '';
  }

  const [date, time] = value.split(' ');
  if (!time) {
    return date;
  }
  return `${date} ${time.slice(0, 5)}`;
}

function stateLabel(state) {
  if (!state) {
    return 'ready';
  }
  return state;
}

function createTextElement(tagName, className, text) {
  const element = document.createElement(tagName);
  if (className) {
    element.className = className;
  }
  element.textContent = text;
  return element;
}

function createIconButton({label, icon, className, onClick}) {
  const button = document.createElement('button');
  button.type = 'button';
  button.className = `icon-button ${className || ''}`.trim();
  button.title = label;
  button.setAttribute('aria-label', label);
  button.innerHTML = `<span class="icon"><i class="${icon}" aria-hidden="true"></i></span>`;
  button.addEventListener('click', onClick);
  return button;
}

function createBadge(text, variant) {
  const badge = createTextElement('span', `task-badge ${variant || ''}`.trim(), text);
  return badge;
}

function createMeta(task) {
  const meta = document.createElement('div');
  meta.className = 'task-meta';

  meta.appendChild(createBadge(stateLabel(task.state), `state-${stateLabel(task.state)}`));

  if (task.priority) {
    meta.appendChild(createBadge(task.priority, 'priority'));
  }

  if (task.context) {
    meta.appendChild(createBadge(task.context, 'context'));
  }

  for (const tag of task.tags || []) {
    meta.appendChild(createBadge(`#${tag}`, 'tag'));
  }

  if (task.date_scheduled) {
    meta.appendChild(createBadge(`Scheduled ${formatDateTime(task.date_scheduled)}`, 'date'));
  }

  if (task.date_due) {
    meta.appendChild(createBadge(`Due ${formatDateTime(task.date_due)}`, 'date'));
  }

  if (task.repetition_due || task.repetition_scheduled) {
    meta.appendChild(createBadge('Repeats', 'repeat'));
  }

  return meta;
}

function createTaskRow(task, actions) {
  const item = document.createElement('article');
  item.className = `task-item is-${stateLabel(task.state)}`;

  const check = document.createElement('button');
  check.type = 'button';
  check.className = 'task-check';
  check.title = task.state === 'completed' ? 'Mark ready' : 'Mark completed';
  check.setAttribute('aria-label', check.title);
  check.innerHTML = '<span class="icon"><i class="fas fa-check" aria-hidden="true"></i></span>';
  check.addEventListener('click', () => {
    actions.onStateChange(task, task.state === 'completed' ? 'ready' : 'completed');
  });

  const content = document.createElement('div');
  content.className = 'task-content';

  const titleLine = document.createElement('div');
  titleLine.className = 'task-title-line';
  titleLine.appendChild(createTextElement('span', 'task-id', `#${task.id}`));
  titleLine.appendChild(createTextElement('h3', 'task-title', task.body));

  content.appendChild(titleLine);
  content.appendChild(createMeta(task));

  if (task.annotation) {
    content.appendChild(createTextElement('p', 'task-annotation', task.annotation));
  }

  const controls = document.createElement('div');
  controls.className = 'task-controls';

  if (task.state !== 'started') {
    controls.appendChild(createIconButton({
      label: 'Mark started',
      icon: 'fas fa-play',
      onClick: () => actions.onStateChange(task, 'started'),
    }));
  }

  if (task.state !== 'ready') {
    controls.appendChild(createIconButton({
      label: 'Mark ready',
      icon: 'fas fa-undo',
      onClick: () => actions.onStateChange(task, 'ready'),
    }));
  }

  controls.appendChild(createIconButton({
    label: 'Delete task',
    icon: 'fas fa-trash',
    className: 'danger',
    onClick: () => actions.onDelete(task),
  }));

  item.appendChild(check);
  item.appendChild(content);
  item.appendChild(controls);

  return item;
}

function flattenGroups(groups) {
  return groups.flatMap((group) => group[1] || []);
}

function updateSummary(groups, visibleTasks) {
  const activeCount = visibleTasks.filter((task) => task.state !== 'completed').length;
  const completedCount = visibleTasks.length - activeCount;
  const contextCount = groups.filter((group) => (group[1] || []).length > 0).length;

  document.getElementById('active-count').textContent = activeCount;
  document.getElementById('completed-count').textContent = completedCount;
  document.getElementById('context-count').textContent = contextCount;
}

function renderGroupTabs(groups, actions) {
  const container = document.getElementById('context-switches');
  clearElement(container);

  const populatedGroups = groups.filter((group) => (group[1] || []).length > 0);
  if (populatedGroups.length <= 1) {
    return;
  }

  populatedGroups.forEach((group, index) => {
    const button = document.createElement('button');
    button.type = 'button';
    button.className = `context-chip ${index === 0 ? 'is-active' : ''}`;
    button.textContent = `${group[0]} (${group[1].length})`;
    button.addEventListener('click', () => {
      container.querySelectorAll('.context-chip').forEach((chip) => chip.classList.remove('is-active'));
      button.classList.add('is-active');
      renderTasks([[group[0], group[1]]], actions, false);
    });
    container.appendChild(button);
  });
}

function renderEmptyState() {
  const emptyState = document.getElementById('empty-state');
  const taskList = document.getElementById('task-list');
  const contextSwitches = document.getElementById('context-switches');
  emptyState.classList.remove('is-hidden');
  taskList.classList.add('is-hidden');
  clearElement(contextSwitches);
}

function renderTasks(groups, actions, updateGroups = true) {
  const taskList = document.getElementById('task-list');
  const emptyState = document.getElementById('empty-state');
  const contextSwitches = document.getElementById('context-switches');
  const visibleTasks = flattenGroups(groups).filter((task) => task.state !== 'completed');

  clearElement(taskList);
  clearElement(contextSwitches);
  updateSummary(groups, flattenGroups(groups));

  if (visibleTasks.length === 0) {
    renderEmptyState();
    return;
  }

  emptyState.classList.add('is-hidden');
  taskList.classList.remove('is-hidden');

  groups.forEach((group) => {
    const tasks = (group[1] || []).filter((task) => task.state !== 'completed');
    if (tasks.length === 0) {
      return;
    }

    const section = document.createElement('section');
    section.className = 'task-section';
    section.appendChild(createTextElement('h2', 'task-section-title', group[0] || 'Tasks'));

    const rows = document.createElement('div');
    rows.className = 'task-rows';
    tasks.forEach((task) => rows.appendChild(createTaskRow(task, actions)));

    section.appendChild(rows);
    taskList.appendChild(section);
  });

  if (updateGroups) {
    renderGroupTabs(groups, actions);
  }
}

export function handleOperation(operation, tasks, actions = {}) {
  const groups = Array.isArray(tasks) ? tasks : [];
  renderTasks(groups, {
    onDelete: actions.onDelete || (() => {}),
    onStateChange: actions.onStateChange || (() => {}),
    onReload: actions.onReload || (() => {}),
  });
}
