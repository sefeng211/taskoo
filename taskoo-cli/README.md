# Requirement
* A terminal supports 256 colors

# Commands
## Add
`taskoo add I am a body` <- Goes to Inbox context by default
`taskoo add I am a body c:Someday` <- Goes to Someday context by default
`taskoo add I am a body c:Someday` +Work <- This is work related
`taskoo add I am a body c:Someday` @Started +Work
`taskoo add I am a body c:Someday` p:taskoo @Started +Work

# Task Attributes
 - Id (Unique)
 - Body
 - Created At
 - Due Date
 - Priority
 - Tags
 - Context (Built-In: Inbox, Someday, Tickler, Gtd)
 - Project
 - State (Not customizable)

State: `Ready`, `Completed`, `Blocked`, `Started`

## List
`task list`: List all tasks.
`task list c:Work`: List all tasks in `Work` context.
`task list c:Work +tag1 +tag2`: List all tasks in `Work` context with `tag1` and `tag2` tags.
`task list c:Work +tag1 ^tag2`: List all tasks in `Work` context with `tag1`, but not `tag2` tags.

## Modify

## Agenda

## Clean
To remove `context` or `tag`

Tasks are sorted by `State: Started` -> `Priority` ->`Created At`
