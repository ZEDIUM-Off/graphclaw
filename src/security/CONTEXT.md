# Security Context

## Local Purpose

`src/security/` handles runtime safety controls: policy enforcement, secret handling, sandbox helpers, audit paths, prompt guardrails, pairing, OTP, and emergency stop controls.

This subtree owns safety constraints for the current runtime. Future GraphClaw context work may depend on these controls as seam consumers, but it must not silently absorb or bypass them.

## What Belongs Here

- runtime safety and policy enforcement;
- secret protection and audit support;
- sandbox and operator-control security flows.

## What Does Not Belong Here

- generic context-resolution logic;
- provider integration detail that belongs in `src/providers/`;
- transport contract ownership that belongs in `src/gateway/`.

## File / Folder Map

- `src/security/mod.rs` - module entry and shared exports
- `src/security/policy.rs` - policy decisions and enforcement helpers
- `src/security/secrets.rs` - secret storage and redaction helpers
- `src/security/audit.rs` - audit logging support
- `src/security/pairing.rs`, `otp.rs`, `estop.rs` - operator-control security flows
- `src/security/landlock.rs`, `bubblewrap.rs`, `firejail.rs`, `docker.rs` - sandbox/environment helpers
- `src/security/prompt_guard.rs`, `detect.rs`, `domain_matcher.rs`, `leak_detector.rs` - content and risk checks

## Go Here For

- Policy or permission logic: `src/security/policy.rs`
- Secret storage/redaction: `src/security/secrets.rs`
- Audit output: `src/security/audit.rs`
- Pairing, OTP, or estop behavior: matching control file
- Sandbox integration details: the relevant backend helper file

## Current State

This is one of the highest-risk inherited runtime areas. It contains real safety controls used by the current system, not just future-facing placeholders.

It should be described as the policy and protection boundary around the runtime, not as a future graph-native policy engine already in place or as the Graph Engine itself.

## Mermaid Map

```mermaid
graph TD
    SecurityMod["mod.rs"] --> Policy["policy.rs"]
    SecurityMod --> Secrets["secrets.rs"]
    SecurityMod --> Audit["audit.rs"]
    SecurityMod --> Guards["prompt and leak guards"]
    SecurityMod --> Controls["pairing otp estop"]
    SecurityMod --> Sandbox["landlock bubblewrap firejail docker"]
    Tools["tools"] --> Policy
    Runtime["runtime"] --> Policy
    Gateway["gateway"] --> Policy
    Agent["agent"] --> Guards
    Policy --> Audit
    Secrets --> Audit
```

## Current Dependency Direction

- Called by execution-sensitive paths in `src/tools/`, `src/runtime/`, `src/gateway/`, and agent-driven workflows that require approval or protection checks.
- Constrains how current and future `ContextPack`-driven actions can be executed, persisted, or exposed.
- Emits or shapes audit-relevant information that may later intersect with `ResolutionTrace`, while remaining a distinct safety layer.

## Sequential Enforcement Path

```mermaid
flowchart TD
    Step1["1. Gateway, tools, runtime, or agent path requests a sensitive action"]
    Step2["2. src/security/mod.rs routes the request to the relevant control path"]
    Step3["3. policy.rs evaluates permissions, constraints, and execution rules"]
    Step4["4. prompt or leak guards inspect content-risk conditions when needed"]
    Step5["5. secrets.rs redacts or protects sensitive material when applicable"]
    Decision{"6. Is the action allowed under current controls?"}
    Step7["7. audit.rs records the decision and relevant security context"]
    Step8["8. Sandbox or operator-control helpers enforce the approved path"]
    Step9["9. Caller receives allow, deny, or stopped result"]

    Step1 --> Step2
    Step2 --> Step3
    Step3 --> Step4
    Step4 --> Step5
    Step5 --> Decision
    Decision -->|yes| Step7
    Decision -->|no| Step7
    Step7 --> Step8
    Step8 --> Step9
```

## Routing

- transport auth and session protocols belong in `src/gateway/`
- runtime adapter constraints belong in `src/runtime/`
- stable policy-facing GraphClaw concepts belong in `docs/architecture/`

## GraphClaw Evolution Note

Do not describe this folder as if GraphClaw already ships a new graph-native policy engine. Current safety behavior is implemented through inherited runtime controls here.

Today, this area contributes to the model by enforcing the policy boundary around context-driven actions rather than by defining context semantics itself. It is a current runtime owner and future seam consumer, not the Graph Engine.

## Constraints / Cautions

- Do not weaken protections casually.
- Security changes usually require tests, docs, and careful threat reasoning.
- Keep policy, storage, auditing, and sandbox concerns distinct.

## References

- `src/runtime/CONTEXT.md` - execution boundary
- `src/gateway/CONTEXT.md` - transport and external-session boundary
- `docs/architecture/graph-context-engine.md` - target model whose future policies must still pass through explicit safety controls

## How Agents Should Work Here

Read the exact control path end to end before editing. Favor explicit reasoning, preserve secure defaults, add or update tests for behavior changes, and call out residual risk if a change cannot be fully verified.
