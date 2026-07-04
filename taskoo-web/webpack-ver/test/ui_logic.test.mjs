import assert from 'node:assert/strict';

import {SERVER_ENDPOINT_MAPPING} from '../src/consts.mjs';
import {
  buildBulkModificationCommand,
  navCounts,
  pruneSelectionForVisibleTasks,
  tagView,
} from '../src/ui_logic.mjs';

const tests = [];

function test(name, fn) {
  tests.push({name, fn});
}

test('delete endpoint uses the key called by the UI', () => {
  assert.equal(SERVER_ENDPOINT_MAPPING.delete, '/api/delete');
  assert.equal(SERVER_ENDPOINT_MAPPING.del, undefined);
});

test('bulk modify command includes multiple ids and supported fields', () => {
  const command = buildBulkModificationCommand([12, 15, 19], {
    context: 'Deep Work',
    tags: 'next, waiting on Bob',
    remove_tags: 'old tag',
    state: 'started',
    priority: 'H',
    due: '2026-07-10',
    due_repeat: 'weekly',
    scheduled: '2026-07-08',
    scheduled_repeat: 'daily',
  });

  assert.equal(
    command,
    '12 15 19 c:Deep-Work +next +waiting-on-Bob ~old-tag @started pri:H d:2026-07-10+weekly s:2026-07-08+daily',
  );
});

test('bulk modify command can represent delete-independent date-only updates', () => {
  assert.equal(
    buildBulkModificationCommand([4, 5], {due: '2026-08-01'}),
    '4 5 d:2026-08-01',
  );
});

test('selection is pruned to tasks still visible after reload', () => {
  const selected = new Set([1, 2, 3]);
  const pruned = pruneSelectionForVisibleTasks(selected, [{id: 2}, {id: 4}]);

  assert.deepEqual([...pruned], [2]);
});

test('nav counts are computed from global tasks, not current view tasks', () => {
  const tasks = [
    {id: 1, context: 'inbox', state: 'ready'},
    {id: 2, context: 'work', state: 'started'},
    {id: 3, context: 'work', state: 'blocked'},
    {id: 4, context: 'inbox', state: 'completed'},
  ];
  const agendaTasks = [{id: 1}, {id: 2}];

  assert.deepEqual(navCounts(tasks, agendaTasks), {
    inbox: 1,
    agenda: 2,
    all: 4,
    started: 1,
    blocked: 1,
    completed: 1,
  });
});

test('tag sidebar entries map to tag query views', () => {
  assert.deepEqual(tagView('taskoo'), {
    view: 'tag:taskoo',
    title: '#taskoo',
    subtitle: 'Tagged tasks',
    query: '+taskoo',
  });
});

let passed = 0;
for (const {name, fn} of tests) {
  try {
    fn();
    passed += 1;
    console.log(`ok - ${name}`);
  } catch (error) {
    console.error(`not ok - ${name}`);
    throw error;
  }
}

console.log(`${passed} tests passed`);
