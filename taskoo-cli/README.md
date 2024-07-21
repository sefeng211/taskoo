# Taskoo CLI
The commandline interface for [Taskoo](https://github.com/sefeng211/taskoo) written in Rust and support
all Taskoo functionalities.

# Installation
## Release Binary
TODO
## Local Compile

# Usage
## Add

To create a task:
`taskoo add I am a body`

Many other options can be provided
`taskoo add "I am a body" c:<context>`: To change the context
`taskoo add "I am a body" +tag1 +tag2`: To add the tags
`taskoo add "I am a body" -a`: To provide an annotation

Pro tip: Always use the [info](#Info) command to show the detailed information
about tasks.

## List

To list all tasks
`taskoo list -a`

To list all (except completed) tasks
`taskoo list`

To list all (except completed) tasks in work context
`taskoo list c:work`

To list all (except completed) tasks in work context with tag1 and tag2
`taskoo list c:work +tag1 +tag2`

To list all (except completed) tasks in work context with tag1 and not tag2
`task list c:Work +tag1 ^tag2`

## Modify

## Agenda

## Clean
To remove `context` or `tag`

Tasks to use. are sorted by `State: Started` -> `Priority` ->`Created At`

## Block
## Complete
## Start
## Ready

## Delete

## Info

## Review
## Clean
