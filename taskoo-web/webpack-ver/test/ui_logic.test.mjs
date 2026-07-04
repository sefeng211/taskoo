import assert from 'node:assert/strict';

import {SERVER_ENDPOINT_MAPPING} from '../src/consts.mjs';
import {
  buildBulkModificationCommand,
  navCounts,
  priorityLabel,
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

test('bulk modify command prefixes command options with multiple task ids', () => {
  const command = buildBulkModificationCommand(
    [12, 15, 19],
    '+next ~oldtag c:work @started pri:H d:2026-07-10 s:2026-07-08',
  );

  assert.equal(
    command,
    '12 15 19 +next ~oldtag c:work @started pri:H d:2026-07-10 s:2026-07-08',
  );
});

test('bulk modify command trims empty space and can represent date-only updates', () => {
  assert.equal(
    buildBulkModificationCommand([4, 5], '  d:2026-08-01  '),
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

test('priority badges use explicit labels', () => {
  assert.equal(priorityLabel('h'), 'Pri:H');
  assert.equal(priorityLabel('L'), 'Pri:L');
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
