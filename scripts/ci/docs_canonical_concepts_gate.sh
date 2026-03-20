#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

terms=(
  "Set"
  "ResolvedSet"
  "View"
  "SessionWindow"
  "ThinkingContext"
  "ContextPack"
  "ContextMutationProposal"
  "ResolutionTrace"
  "TaskIntent"
  "StrategyResolution"
  "AgentPackage"
)

canonical_files=(
  "docs/architecture/concepts/definition-governance.md"
  "docs/architecture/concepts/views-and-sets.md"
  "docs/architecture/interfaces/session-window-interface.md"
  "docs/architecture/interfaces/context-pack-interface.md"
  "docs/architecture/interfaces/strategy-resolver-interface.md"
  "docs/architecture/concepts/graph-context-engine.md"
  "docs/architecture/concepts/glossary.md"
)

for file in "${canonical_files[@]}"; do
  if [ ! -f "$file" ]; then
    echo "Missing canonical docs file: $file"
    exit 1
  fi
done

violations=0

check_pattern_in_file() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if rg -n "$pattern" "$file" >/tmp/graphclaw_canonical_gate_match.$$ 2>/dev/null; then
    echo "Forbidden $label in $file"
    cat /tmp/graphclaw_canonical_gate_match.$$
    violations=1
  fi
  rm -f /tmp/graphclaw_canonical_gate_match.$$
}

while IFS= read -r file; do
  [ -n "$file" ] || continue
  for term in "${terms[@]}"; do
    check_pattern_in_file "$file" "^[-*] \`$term\`: [^\\[]" "inline concept definition"
    check_pattern_in_file "$file" "^### \`$term\`$" "canonical-style concept heading"
  done
done < <(find . -name AGENTS.md -o -name CONTEXT.md | sort)

for term in "${terms[@]}"; do
  check_pattern_in_file "docs/architecture/concepts/glossary.md" "^### \`$term\`$" "glossary duplicate definition"
done

if ! rg -n "## Registre Canonique Initial" docs/architecture/concepts/definition-governance.md >/dev/null 2>&1; then
  echo "Missing canonical registry section in docs/architecture/concepts/definition-governance.md"
  violations=1
fi

if [ "$violations" -ne 0 ]; then
  echo "Canonical concept gate failed."
  exit 1
fi

echo "Canonical concept gate passed."
