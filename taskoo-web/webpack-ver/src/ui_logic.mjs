export function escapeToken(value) {
  return (value || '').trim().replace(/\s+/g, '-');
}

export function tagTokens(value, prefix) {
  return (value || '')
    .split(',')
    .map((tag) => escapeToken(tag))
    .filter(Boolean)
    .map((tag) => `${prefix}${tag}`);
}

export function priorityLabel(priority) {
  return `Pri:${String(priority).toUpperCase()}`;
}

function localDateKey(date) {
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
}

function startOfLocalDay(date) {
  return new Date(date.getFullYear(), date.getMonth(), date.getDate());
}

function parseAgendaDate(value) {
  if (!value) {
    return null;
  }

  const [datePart] = String(value).split(' ');
  const [year, month, day] = datePart.split('-').map((part) => Number(part));
  if (![year, month, day].every(Number.isFinite)) {
    return null;
  }

  return new Date(year, month - 1, day);
}

export function agendaDateLabel(value, today = new Date()) {
  const date = parseAgendaDate(value);
  if (!date) {
    return value || '';
  }

  const diffDays = Math.round((startOfLocalDay(date) - startOfLocalDay(today)) / 86400000);
  if (diffDays === 0) {
    return `${localDateKey(date)} (today)`;
  }

  const distance = Math.abs(diffDays);
  const direction = diffDays > 0 ? 'from now' : 'ago';
  return `${localDateKey(date)} (${distance} day${distance === 1 ? '' : 's'} ${direction})`;
}

export function pruneSelectionForVisibleTasks(selectedIds, tasks) {
  const visibleIds = new Set(tasks.map((task) => task.id));
  return new Set([...selectedIds].filter((id) => visibleIds.has(id)));
}

export function uniqueTasksById(tasks) {
  const unique = new Map();
  tasks.forEach((task) => {
    if (task && !unique.has(task.id)) {
      unique.set(task.id, task);
    }
  });
  return [...unique.values()];
}

export function visibleTasksForView(tasks, view) {
  if (view === 'completed') {
    return tasks;
  }
  return tasks.filter((task) => task.state !== 'completed');
}

export function buildBulkModificationCommand(taskIds, values) {
  const tokens = [taskIds.join(' ')];
  if (typeof values === 'string') {
    tokens.push(values.trim());
    return tokens.filter(Boolean).join(' ');
  }

  const context = escapeToken(values.context);
  if (context) tokens.push(`c:${context}`);
  tokens.push(...tagTokens(values.tags, '+'));
  tokens.push(...tagTokens(values.remove_tags, '~'));
  if (values.state) tokens.push(`@${values.state}`);
  if (values.priority) tokens.push(`pri:${values.priority}`);
  if (values.due) tokens.push(`d:${values.due}${values.due_repeat ? `+${escapeToken(values.due_repeat)}` : ''}`);
  if (values.scheduled) tokens.push(`s:${values.scheduled}${values.scheduled_repeat ? `+${escapeToken(values.scheduled_repeat)}` : ''}`);
  return tokens.filter(Boolean).join(' ');
}

export function navCounts(tasks, agendaTasks) {
  return {
    inbox: tasks.filter((task) => task.context === 'inbox' && task.state !== 'completed').length,
    agenda: agendaTasks.length,
    all: tasks.length,
    started: tasks.filter((task) => task.state === 'started').length,
    blocked: tasks.filter((task) => task.state === 'blocked').length,
    completed: tasks.filter((task) => task.state === 'completed').length,
  };
}

export function tagView(tag) {
  return {
    view: `tag:${tag}`,
    title: `#${tag}`,
    subtitle: 'Tagged tasks',
    query: `+${tag}`,
  };
}
