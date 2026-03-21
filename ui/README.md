# GraphClaw UI

This directory hosts the future GraphClaw web interface as a separate Vue project.

Current scope:

- Vue 3 application
- `pnpm` package workflow
- Vite+ intended as the build surface
- only the graph playground is implemented in this migration step

Implementation note:

- the package manifest currently aliases `vite` to `vite-plus` based on VoidZero's public framing of Vite+ as a drop-in superset of Vite. If the preview package naming or install guidance changes, update `package.json` to match the official guide.
- the dev proxy targets the GraphClaw gateway default on `127.0.0.1:42617` unless `GATEWAY_ORIGIN` overrides it.
