# Python Tools Context

## Scope

Python implementations of tool behaviors exposed by the companion package.

## File Map

- `__init__.py` - subpackage exports
- `base.py` - shared tool abstractions
- `shell.py`, `file.py`, `memory.py`, `web.py` - concrete tool implementations

## Routing

Shared interfaces start in `base.py`; concrete tool modules plug into that contract and are exercised from the package-level runtime and tests.

## Current State

This subtree is an inherited tool surface with stable names and limited scope.

## GraphClaw Relevance

Tool contracts here are a migration boundary: the repo can evolve context scaffolding without casually breaking Python-side operator utilities.

## Cautions

- Preserve tool contracts unless the task explicitly changes them.
- Avoid scattering shared abstractions across concrete tool files when `base.py` should own them.

## Agent Guidance

- Add tests in `python/tests/` before changing behavior here.
- Keep new tool modules narrowly scoped and consistent with the existing package layout.
