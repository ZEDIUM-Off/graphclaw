# Docs Reference Context

## Local Purpose

This subtree is the exact-lookup layer for current interfaces. It exists so operators, contributors, and integrators can find precise information about CLI commands, API/config surfaces, and SOP behavior without digging through code or narrative docs.

## What Belongs Here

- exact command, config, provider, channel, and SOP reference;
- stable lookup material for current interfaces;
- concise reference entrypoints that route readers into the correct detailed page.

## What Does Not Belong Here

- broad migration narrative or contributor workflow policy;
- speculative interfaces that are not implemented;
- operator runbooks that belong in `docs/ops/`.

## File Map

- `README.md` - reference entrypoint
- `api/` - config, provider, and channel reference
- `cli/` - command reference
- `sop/` - SOP syntax and subsystem reference

## Routing

- exact commands go to `docs/reference/cli/`
- config keys, provider behavior, and channels go to `docs/reference/api/`
- SOP syntax and related subsystem behavior go to `docs/reference/sop/`
- broader operating guidance goes to `docs/ops/`

## References

- `docs/CONTEXT.md` - docs-tree guidance
- `docs/ops/CONTEXT.md` - operations boundary
- `docs/setup-guides/CONTEXT.md` - install/setup boundary

## Current Inherited State

Reference docs remain tied to inherited `zeroclaw` surfaces because those commands, config structures, and subsystem behaviors still define the current implementation. Reference accuracy matters more than migration messaging density.

## GraphClaw Migration Relationship

This subtree should only change names, parameters, or interface descriptions when the underlying surface changes. It can acknowledge GraphClaw migration context, but it must not pre-document future interfaces as current reference.

## Cautions

- reference docs must stay exact and implementation-aligned
- do not soften inherited names unless the real interface changed
- keep cross-links clear so readers do not confuse narrative docs with authoritative reference

## Agent Workflow

1. Confirm the requested change concerns an actual interface or reference route.
2. Check the current command, config, or subsystem behavior before editing.
3. Update the narrowest reference page that owns the fact.
4. Preserve inherited terminology until the technical surface actually migrates.
