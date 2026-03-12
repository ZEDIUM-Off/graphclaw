# Docs Security Context

## Local Purpose

This subtree documents the repository's security posture: current hardening and auditing guidance, security model notes, and roadmap material that must be clearly separated from implemented protections.

## What Belongs Here

- current security behavior and hardening guidance;
- audit logging and sandboxing documentation;
- clearly labeled roadmap material for planned security work.

## What Does Not Belong Here

- vague reassurance language that hides limitations;
- general setup or ops walkthroughs better owned elsewhere;
- future protections described as if already enforced.

## File Map

- `README.md` - security docs entrypoint
- `sandboxing.md` - sandbox model and limits
- `audit-logging.md` - audit visibility and logging guidance
- `security-roadmap.md` - planned security work
- `agnostic-security.md` and `frictionless-security.md` - conceptual framing pages
- `matrix-e2ee-guide.md` - specific security-related integration guidance

## Routing

- current security controls and limitations belong here
- operational response or runtime troubleshooting belongs in `docs/ops/`
- installation of services belongs in `docs/setup-guides/`
- exact config knobs may need cross-reference from `docs/reference/api/`

## References

- `docs/CONTEXT.md` - docs-tree framing
- `src/security/CONTEXT.md` - source-side security boundary
- `.github/codeql/CONTEXT.md` - GitHub code scanning automation boundary

## Current Inherited State

This area documents security behavior for the current inherited runtime. Some pages also contain roadmap material. Both are valid, but they must be distinguished clearly because GraphClaw migration is not the same thing as shipped security posture.

## GraphClaw Migration Relationship

Migration may eventually change trust boundaries or documentation structure, but this subtree must remain truthful about protections and gaps that exist today. GraphClaw framing can add context; it must not dilute security precision.

## Cautions

- always separate current behavior from roadmap
- avoid softening caveats for readability or branding consistency
- treat security claims as high-risk statements that require verification

## Agent Workflow

1. Identify whether the target statement is current-state security guidance or roadmap.
2. Verify the claim against existing implementation or authoritative policy before editing.
3. Preserve inherited terminology when it maps to real current controls.
4. Prefer explicit caveats over ambiguous wording.
