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

export function pruneSelectionForVisibleTasks(selectedIds, tasks) {
  const visibleIds = new Set(tasks.map((task) => task.id));
  return new Set([...selectedIds].filter((id) => visibleIds.has(id)));
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
