# `CONTEXT.md` Template

Use this template when creating or rewriting a directory-level `CONTEXT.md`.

The goal is not to summarize filenames mechanically. The goal is to document boundaries, ownership, routing, and migration truth for that part of the repository.

## Required sections

Every local `CONTEXT.md` should answer these questions:

1. What is this area for
2. What belongs here
3. What does not belong here
4. Which files or subtrees matter first
5. How should work be routed from here
6. Which references define the local truth
7. What inherited reality still applies
8. How this area relates to GraphClaw migration
9. What mistakes or boundary leaks to avoid
10. How an agent should work in this area

## Recommended skeleton

```md
# <Area Name> Context

## Local Purpose

<One short paragraph describing why this area exists.>

## What Belongs Here

- <responsibility one>
- <responsibility two>
- <responsibility three>

## What Does Not Belong Here

- <explicit non-responsibility>
- <neighboring concern that belongs elsewhere>
- <things to avoid documenting or implementing here>

## File Map

- `<file-or-dir>` - <why it matters>
- `<file-or-dir>` - <why it matters>
- `<file-or-dir>` - <why it matters>

## Routing

- <change type> belongs in `<path>/`
- <other change type> belongs in `<path>/`
- <if work crosses a boundary, read these docs first>

## References

- `<path>` - <why this is authoritative>
- `<path>` - <why this is authoritative>
- `<path>` - <why this is authoritative>

## Current Inherited State

<State what still comes from the inherited `zeroclaw` baseline and why that is still current truth here.>

## GraphClaw Migration Relationship

<Explain how this area supports migration without pretending the target architecture already exists here.>

## Cautions

- <common misconception>
- <boundary that must stay explicit>
- <truthfulness rule for this area>

## Agent Workflow

1. <first reading or verification step>
2. <how to classify the change>
3. <what to preserve or avoid>
4. <required validation or follow-up>
```

## Writing rules

- Prefer boundary statements over broad narrative.
- Name neighboring areas explicitly when they own adjacent responsibilities.
- Keep inherited implementation truth separate from target architecture intent.
- If the area changed shape, update the local file map and routing guidance in the same patch.
- If the directory is only a thin wrapper around deeper subtrees, say so directly.
