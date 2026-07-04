# Taskoo CLI Usage Guide

Taskoo is a local-first task manager built around GTD-style capture, clarify, organize, review, and engage workflows. The CLI is the most direct interface to the core model: tasks live in contexts, can have tags, dates, priorities, states, annotations, and optional dependencies.

This guide documents the practical command-line workflow and the command syntax supported by the current implementation.

## Mental Model

Taskoo is optimized for a GTD workflow:

1. Capture quickly into `inbox`.
2. Clarify tasks by assigning context, tags, schedule, due dates, and priority.
3. Engage from focused lists such as context lists, today agenda, started tasks, or tag searches.
4. Review the inbox regularly and move tasks into the right context.
5. Complete or delete stale work.

The default context is `inbox`. Context and tag names are normalized to lowercase by the core.

## Task Fields

A task can have:

- `id`: numeric task id
- `body`: task title/body
- `context`: one list/context, default `inbox`
- `tags`: zero or more tags
- `priority`: `H`, `M`, or `L`
- `state`: usually `ready`, `started`, `blocked`, or `completed`
- `date_due`: due date
- `date_scheduled`: scheduled date
- `repetition_due`: recurrence used when completing due tasks
- `repetition_scheduled`: recurrence used when completing scheduled tasks
- `annotation`: longer note text
- `parent_task_ids`: dependency ids, currently lightly used

## Command Syntax

Taskoo uses compact tokens inspired by todo.txt-style task managers.

| Token | Meaning | Example |
| --- | --- | --- |
| `c:<context>` | Set or filter by context | `c:work` |
| `+<tag>` | Add or filter by tag | `+phone` |
| `^<tag>` | Exclude tag in list queries | `^waiting` |
| `~<tag>` | Remove tag in modify queries | `~oldtag` |
| `d:<date>` | Due date | `d:2026-07-10` |
| `s:<date>` | Scheduled date | `s:2026-07-08` |
| `d:<date>+<repeat>` | Due date with recurrence | `d:2026-07-10+weekly` |
| `s:<date>+<repeat>` | Schedule with recurrence | `s:2026-07-08+daily` |
| `pri:<priority>` | Priority | `pri:H` |
| `@<state>` | State | `@started` |
| `dep:<ids>` | Parent dependencies | `dep:12,14` |
| `<start>..<end>` | Task id range | `3..7` |

Dates are parsed by the core date parser. ISO dates such as `YYYY-MM-DD` are the safest format.

## Capture

Add a simple task:

```sh
taskoo add Buy milk
```

Add directly to a context:

```sh
taskoo add Call dentist c:personal
```

Add tags:

```sh
taskoo add Follow up with Alex c:work +email +waiting
```

Add priority:

```sh
taskoo add Renew passport c:personal pri:H
```

Add due and scheduled dates:

```sh
taskoo add Submit report c:work d:2026-07-10 s:2026-07-08
```

Add a recurring task:

```sh
taskoo add Pay rent c:home d:2026-08-01+monthly
```

Add a task with an annotation. This opens your editor:

```sh
taskoo add -a Research trip options c:personal +travel
```

Best practice: capture quickly and keep most new tasks in `inbox` unless the context is obvious.

## List Tasks

List active tasks, grouped by context:

```sh
taskoo list
```

List all tasks, including completed:

```sh
taskoo list -a
```

List one context:

```sh
taskoo list c:work
```

List tasks with tags:

```sh
taskoo list c:work +phone
taskoo list c:work +phone +urgent
```

Exclude a tag:

```sh
taskoo list c:work ^waiting
```

List by due or scheduled date:

```sh
taskoo list d:2026-07-10
taskoo list s:2026-07-08
```

Useful GTD lists:

```sh
taskoo list c:inbox
taskoo list c:work +next
taskoo list +waiting
taskoo list c:personal ^someday
```

Note: the current `list` operation supports context, tags, not-tags, due date, scheduled date, and task id lookup internally. State filtering is best handled with the dedicated state commands/workflows or by the web UI.

## Agenda

Agenda shows tasks whose due or scheduled dates fall before the selected day boundary.

Show today:

```sh
taskoo agenda today
```

Show a specific day:

```sh
taskoo agenda 2026-07-10
```

Show a date range:

```sh
taskoo agenda 2026-07-10 2026-07-17
```

Best practice: use `agenda today` for your daily engage view, then use context lists for unscheduled work.

## Modify Tasks

Modify accepts one or more task ids followed by the same field tokens.

Move a task to another context:

```sh
taskoo modify 12 c:work
```

Add tags:

```sh
taskoo modify 12 +next +phone
```

Remove tags:

```sh
taskoo modify 12 ~waiting
```

Set due and scheduled dates:

```sh
taskoo modify 12 d:2026-07-10
taskoo modify 12 s:2026-07-08
```

Set recurrence:

```sh
taskoo modify 12 d:2026-07-10+weekly
taskoo modify 12 s:2026-07-08+daily
```

Set priority:

```sh
taskoo modify 12 pri:H
```

Set state:

```sh
taskoo modify 12 @started
taskoo modify 12 @blocked
taskoo modify 12 @ready
taskoo modify 12 @completed
```

Modify multiple tasks:

```sh
taskoo modify 12 13 14 c:work +next
```

Modify a range:

```sh
taskoo modify 20..25 +review
```

## State Shortcuts

The CLI provides direct commands for the built-in states.

Start tasks:

```sh
taskoo start 12
taskoo start 12 13 14
```

Complete tasks:

```sh
taskoo complete 12
```

Mark tasks ready:

```sh
taskoo ready 12
```

Block tasks:

```sh
taskoo block 12
```

When a recurring task is completed, the core can advance its due or scheduled date based on its recurrence and set it back to `ready`.

## Delete Tasks

Delete one task:

```sh
taskoo delete 12
```

Delete multiple tasks:

```sh
taskoo delete 12 13 14
```

Delete a range:

```sh
taskoo delete 20..25
```

Use deletion for tasks that are no longer meaningful. Use `complete` for work that was actually done.

## Inspect Tasks and Metadata

Show one task:

```sh
taskoo info 12
```

Show one property:

```sh
taskoo info 12
```

The core supports task properties such as:

```text
priority
context
tags
date_created
date_due
date_scheduled
repetition_due
repetition_scheduled
state
annotation
parent_task_ids
```

Show tags:

```sh
taskoo info tag
```

## Review Workflow

Review is designed for GTD inbox clarification. By default it reviews `inbox`.

Review inbox:

```sh
taskoo review
```

Review a specific context:

```sh
taskoo review c:work
```

For each task, review lets you:

- skip it
- delete it
- choose or create a context
- choose or create tags
- set due date
- set scheduled date

Best practice: run review regularly on `inbox`. Keep capture friction low, then clarify later.

## Clean Unused Metadata

Clean removes unused contexts, tags, or custom states. It only offers items with no associated tasks.

```sh
taskoo clean context
taskoo clean tag
taskoo clean state
```

This is useful after reorganizing tags or contexts.

## Suggested Daily Workflow

Morning:

```sh
taskoo agenda today
taskoo list c:inbox
taskoo review
```

Choose active work:

```sh
taskoo list c:work +next
taskoo start 12
```

During the day:

```sh
taskoo add Call Sam c:inbox
taskoo add Draft proposal c:work +next pri:H
taskoo block 14
taskoo complete 12
```

End of day:

```sh
taskoo review
taskoo list +waiting
taskoo agenda tomorrow
```

## Recommended Contexts and Tags

Contexts should describe where or in what mode you can do the work:

```text
inbox
work
home
personal
errands
computer
phone
```

Tags should describe project, workflow, or status:

```text
+next
+waiting
+someday
+review
+email
+phone
+deep
```

Keep contexts few and stable. Use tags more freely.

## Database Configuration

Taskoo reads its database config from:

```text
~/.config/taskoo/config
```

Example:

```text
db_path=/absolute/path/to/tasks.db
```

If the config file does not exist, Taskoo creates a default database at:

```text
~/.config/taskoo/tasks.db
```

## Caveats

- Context and tag names are lowercased by the core.
- Task body parsing stops once option tokens begin, so put the body first when adding tasks.
- `list` does not currently filter by state token in the same way `modify` sets state.
- `parent_task_ids` exists, but dependency workflows are still minimal.
- `clean` only deletes metadata that has no associated tasks.
- The CLI README is older and incomplete; this guide is based on the command implementations and core parser.
