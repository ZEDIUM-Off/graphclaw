SHELL := /usr/bin/env bash
.DEFAULT_GOAL := help

CARGO ?= cargo
NPM ?= npm
PYTHON ?= python3
WEB_DIR ?= web
TAG ?=
ORIGIN_REMOTE ?= origin
UPSTREAM_REMOTE ?= upstream
UPSTREAM_BRANCH ?= master
BASE_BRANCH ?= master
SYNC_DATE ?= $(shell date +%Y%m%d)
SYNC_BRANCH ?= resync/$(UPSTREAM_REMOTE)-$(UPSTREAM_BRANCH)-$(SYNC_DATE)
BACKUP_BRANCH ?= backup/pre-resync-$(SYNC_DATE)
UPSTREAM_URL ?=
RELEASE_TAG ?=
RELEASE_BRANCH ?= resync/release-$(RELEASE_TAG)-$(SYNC_DATE)
MEMGRAPH_DIR ?= memgraph

.PHONY: help \
	fmt fmt-check clippy clippy-strict lint lint-strict \
	test test-component test-integration test-system test-live test-manual \
	build build-release build-release-fast bench check \
	run run-agent run-daemon run-gateway run-doctor run-onboard \
	docs docs-links \
	web-dev web-build web-preview \
	python-test \
	ci ci-build-image ci-shell ci-lint ci-lint-strict ci-lint-delta ci-test ci-build ci-security ci-docker-smoke \
	dev-up dev-down dev-shell dev-agent dev-logs dev-build dev-clean \
	memgraph-up memgraph-down memgraph-logs memgraph-shell \
	release-tag \
	sync-help sync-check-clean sync-remotes sync-add-upstream sync-fetch sync-status sync-divergence \
	sync-backup sync-branch sync-merge-upstream sync-validate \
	sync-release-help sync-release-list sync-release-latest sync-release-divergence \
	sync-release-branch sync-merge-release

help:
	@printf '%s\n' \
		'GraphClaw Makefile targets' \
		'' \
		'Core Rust' \
		'  make fmt                Format Rust code' \
		'  make fmt-check          Check Rust formatting' \
		'  make lint               Run repo rust quality gate' \
		'  make lint-strict        Run strict clippy gate' \
		'  make test               Run full Rust test suite' \
		'  make build              Debug build' \
		'  make build-release      Release build' \
		'  make build-release-fast Faster optimized build' \
		'  make bench              Run benches' \
		'' \
		'Run commands' \
		'  make run-agent          cargo run -- agent' \
		'  make run-daemon         cargo run -- daemon' \
		'  make run-gateway        cargo run -- gateway' \
		'  make run-doctor         cargo run -- doctor' \
		'  make run-onboard        cargo run -- onboard' \
		'' \
		'Docs and web' \
		'  make docs               Run docs quality gate' \
		'  make docs-links         Run docs links gate' \
		'  make web-dev            Run Vite dev server' \
		'  make web-build          Build web app' \
		'  make web-preview        Preview web app build' \
		'' \
		'Docker/local CI' \
		'  make ci                 Run full local CI in Docker' \
		'  make ci-lint            Run Docker lint gate' \
		'  make ci-test            Run Docker test suite' \
		'  make ci-build           Run Docker release build' \
		'  make ci-security        Run Docker audit + deny' \
		'  make ci-docker-smoke    Build runtime image smoke test' \
		'' \
		'Dev environment' \
		'  make dev-up             Start local dev containers' \
		'  make dev-shell          Enter sandbox container' \
		'  make dev-agent          Enter agent container' \
		'  make dev-logs           Follow container logs' \
		'  make dev-build          Rebuild dev containers' \
		'  make dev-down           Stop dev containers' \
		'' \
		'Memgraph (graph backend)' \
		'  make memgraph-up        Start Memgraph and Memgraph Lab' \
		'  make memgraph-down      Stop Memgraph stack' \
		'  make memgraph-logs      Follow Memgraph logs' \
		'  make memgraph-shell     Exec into Memgraph container' \
		'' \
		'Upstream resync protocol' \
		'  make sync-help          Show recommended resync sequence' \
		'  make sync-release-help  Show release-based resync sequence' \
		'  make sync-remotes       Show git remotes' \
		'  make sync-add-upstream UPSTREAM_URL=https://github.com/zeroclaw-labs/zeroclaw.git' \
		'  make sync-fetch         Fetch origin + upstream tags' \
		'  make sync-release-list  List stable upstream tags (vX.Y.Z)' \
		'  make sync-release-latest Show latest stable upstream tag from git tags' \
		'  make sync-status        Show branch, status, and remotes' \
		'  make sync-divergence    Show master vs upstream/master divergence' \
		'  make sync-release-divergence RELEASE_TAG=v0.1.7' \
		'  make sync-backup        Create backup branch from base branch' \
		'  make sync-branch        Create integration branch $(SYNC_BRANCH)' \
		'  make sync-release-branch RELEASE_TAG=v0.1.7' \
		'  make sync-merge-upstream Merge upstream/master into the integration branch' \
		'  make sync-merge-release RELEASE_TAG=v0.1.7' \
		'  make sync-validate      Run docs + Rust validation after conflict resolution' \
		'' \
		'Other' \
		'  make python-test        Run Python tests' \
		'  make release-tag TAG=vX.Y.Z  Cut release tag via script'

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --locked --all-targets -- -D clippy::correctness

clippy-strict:
	$(CARGO) clippy --locked --all-targets -- -D warnings

lint:
	./scripts/ci/rust_quality_gate.sh

lint-strict:
	./scripts/ci/rust_quality_gate.sh --strict

test:
	$(CARGO) test --locked --verbose

test-component:
	$(CARGO) test --test component --locked --verbose

test-integration:
	$(CARGO) test --test integration --locked --verbose

test-system:
	$(CARGO) test --test system --locked --verbose

test-live:
	$(CARGO) test --test live -- --ignored --verbose

test-manual:
	bash tests/manual/test_dockerignore.sh

build:
	$(CARGO) build --locked --verbose

build-release:
	$(CARGO) build --release --locked --verbose

build-release-fast:
	$(CARGO) build --profile release-fast --locked --verbose

bench:
	$(CARGO) bench --bench agent_benchmarks --locked

check: lint test docs

run: run-agent

run-agent:
	$(CARGO) run --locked -- agent

run-daemon:
	$(CARGO) run --locked -- daemon

run-gateway:
	$(CARGO) run --locked -- gateway

run-doctor:
	$(CARGO) run --locked -- doctor

run-onboard:
	$(CARGO) run --locked -- onboard

docs:
	./scripts/ci/docs_quality_gate.sh

docs-links:
	./scripts/ci/docs_links_gate.sh

web-dev:
	$(NPM) --prefix $(WEB_DIR) run dev

web-build:
	$(NPM) --prefix $(WEB_DIR) run build

web-preview:
	$(NPM) --prefix $(WEB_DIR) run preview

python-test:
	$(PYTHON) -m pytest python/tests

ci:
	./dev/ci.sh all

ci-build-image:
	./dev/ci.sh build-image

ci-shell:
	./dev/ci.sh shell

ci-lint:
	./dev/ci.sh lint

ci-lint-strict:
	./dev/ci.sh lint-strict

ci-lint-delta:
	./dev/ci.sh lint-delta

ci-test:
	./dev/ci.sh test

ci-build:
	./dev/ci.sh build

ci-security:
	./dev/ci.sh security

ci-docker-smoke:
	./dev/ci.sh docker-smoke

dev-up:
	./dev/cli.sh up

dev-down:
	./dev/cli.sh down

dev-shell:
	./dev/cli.sh shell

dev-agent:
	./dev/cli.sh agent

dev-logs:
	./dev/cli.sh logs

dev-build:
	./dev/cli.sh build

dev-clean:
	./dev/cli.sh clean

memgraph-up:
	cd $(MEMGRAPH_DIR) && docker compose up -d

memgraph-down:
	cd $(MEMGRAPH_DIR) && docker compose down

memgraph-logs:
	cd $(MEMGRAPH_DIR) && docker compose logs -f

memgraph-shell:
	cd $(MEMGRAPH_DIR) && docker compose exec memgraph bash

release-tag:
	@test -n "$(TAG)" || (echo "Usage: make release-tag TAG=vX.Y.Z" && exit 1)
	./scripts/release/cut_release_tag.sh "$(TAG)"

sync-help:
	@printf '%s\n' \
		'GraphClaw upstream resync protocol' \
		'' \
		'Recommended sequence:' \
		'  1. make sync-status' \
		'  2. make sync-fetch' \
		'  3. make sync-divergence' \
		'  4. make sync-backup' \
		'  5. make sync-branch' \
		'  6. make sync-merge-upstream' \
		'  7. resolve conflicts manually' \
		'  8. make sync-validate' \
		'' \
		'Defaults:' \
		"  ORIGIN_REMOTE=$(ORIGIN_REMOTE)" \
		"  UPSTREAM_REMOTE=$(UPSTREAM_REMOTE)" \
		"  UPSTREAM_BRANCH=$(UPSTREAM_BRANCH)" \
		"  BASE_BRANCH=$(BASE_BRANCH)" \
		"  SYNC_BRANCH=$(SYNC_BRANCH)" \
		"  BACKUP_BRANCH=$(BACKUP_BRANCH)"

sync-release-help:
	@printf '%s\n' \
		'GraphClaw stable-release resync protocol' \
		'' \
		'Recommended sequence:' \
		'  1. make sync-status' \
		'  2. make sync-fetch' \
		'  3. make sync-release-list' \
		'  4. make sync-release-divergence RELEASE_TAG=v0.1.7' \
		'  5. make sync-backup' \
		'  6. make sync-release-branch RELEASE_TAG=v0.1.7' \
		'  7. make sync-merge-release RELEASE_TAG=v0.1.7' \
		'  8. resolve conflicts manually' \
		'  9. make sync-validate' \
		'' \
		'Notes:' \
		'  - Stable releases are matched as tags like v0.1.7.' \
		'  - This workflow syncs against a tagged release, not upstream/master.' \
		'' \
		'Defaults:' \
		"  ORIGIN_REMOTE=$(ORIGIN_REMOTE)" \
		"  UPSTREAM_REMOTE=$(UPSTREAM_REMOTE)" \
		"  BASE_BRANCH=$(BASE_BRANCH)" \
		"  RELEASE_TAG=$(RELEASE_TAG)" \
		"  RELEASE_BRANCH=$(RELEASE_BRANCH)" \
		"  BACKUP_BRANCH=$(BACKUP_BRANCH)"

sync-check-clean:
	@git diff --quiet || (echo 'Working tree has unstaged changes; clean or commit them first.' && exit 1)
	@git diff --cached --quiet || (echo 'Index has staged changes; commit or unstage them first.' && exit 1)

sync-remotes:
	git remote -v

sync-add-upstream:
	@test -n "$(UPSTREAM_URL)" || (echo "Usage: make sync-add-upstream UPSTREAM_URL=https://github.com/zeroclaw-labs/zeroclaw.git" && exit 1)
	@if git remote get-url "$(UPSTREAM_REMOTE)" >/dev/null 2>&1; then \
		echo "Remote $(UPSTREAM_REMOTE) already exists:"; \
		git remote get-url "$(UPSTREAM_REMOTE)"; \
	else \
		git remote add "$(UPSTREAM_REMOTE)" "$(UPSTREAM_URL)"; \
		echo "Added remote $(UPSTREAM_REMOTE) -> $(UPSTREAM_URL)"; \
	fi

sync-fetch:
	git fetch "$(ORIGIN_REMOTE)"
	git fetch "$(UPSTREAM_REMOTE)" --tags

sync-release-list:
	@git ls-remote --tags "$(UPSTREAM_REMOTE)" | \
		awk '{print $$2}' | \
		sed 's#refs/tags/##' | \
		sed 's/\^{}$$//' | \
		grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$$' | \
		sort -uV

sync-release-latest:
	@latest="$$(git ls-remote --tags "$(UPSTREAM_REMOTE)" | \
		awk '{print $$2}' | \
		sed 's#refs/tags/##' | \
		sed 's/\^{}$$//' | \
		grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$$' | \
		sort -uV | tail -n 1)"; \
		test -n "$$latest" || (echo "No stable release tags found on $(UPSTREAM_REMOTE)." && exit 1); \
		echo "$$latest"

sync-status:
	@printf 'Current branch: '
	@git branch --show-current
	@git status --short
	@printf '\nRemotes:\n'
	@git remote -v

sync-divergence:
	git log --oneline --left-right --graph "$(BASE_BRANCH)...$(UPSTREAM_REMOTE)/$(UPSTREAM_BRANCH)"
	@printf '\nUpstream-only commits:\n'
	@git log --oneline "$(BASE_BRANCH)..$(UPSTREAM_REMOTE)/$(UPSTREAM_BRANCH)" || true
	@printf '\nGraphClaw-only commits:\n'
	@git log --oneline "$(UPSTREAM_REMOTE)/$(UPSTREAM_BRANCH)..$(BASE_BRANCH)" || true
	@printf '\nDiff summary:\n'
	@git diff --stat "$(BASE_BRANCH)...$(UPSTREAM_REMOTE)/$(UPSTREAM_BRANCH)"

sync-release-divergence:
	@test -n "$(RELEASE_TAG)" || (echo "Usage: make sync-release-divergence RELEASE_TAG=v0.1.7" && exit 1)
	@git rev-parse "$(RELEASE_TAG)^{tag}" >/dev/null 2>&1 || git rev-parse "$(RELEASE_TAG)^{commit}" >/dev/null 2>&1 || \
		(echo "Release tag $(RELEASE_TAG) is not available locally. Run make sync-fetch first." && exit 1)
	git log --oneline --left-right --graph "$(BASE_BRANCH)...$(RELEASE_TAG)"
	@printf '\nRelease-only commits:\n'
	@git log --oneline "$(BASE_BRANCH)..$(RELEASE_TAG)" || true
	@printf '\nGraphClaw-only commits:\n'
	@git log --oneline "$(RELEASE_TAG)..$(BASE_BRANCH)" || true
	@printf '\nDiff summary:\n'
	@git diff --stat "$(BASE_BRANCH)...$(RELEASE_TAG)"

sync-backup: sync-check-clean
	git checkout "$(BASE_BRANCH)"
	git pull "$(ORIGIN_REMOTE)" "$(BASE_BRANCH)"
	git branch "$(BACKUP_BRANCH)"
	@echo "Created backup branch: $(BACKUP_BRANCH)"

sync-branch: sync-check-clean
	git checkout "$(BASE_BRANCH)"
	git checkout -b "$(SYNC_BRANCH)"
	@echo "Created integration branch: $(SYNC_BRANCH)"

sync-release-branch: sync-check-clean
	@test -n "$(RELEASE_TAG)" || (echo "Usage: make sync-release-branch RELEASE_TAG=v0.1.7" && exit 1)
	git checkout "$(BASE_BRANCH)"
	git checkout -b "$(RELEASE_BRANCH)"
	@echo "Created release integration branch: $(RELEASE_BRANCH)"

sync-merge-upstream:
	@if [ "$$(git branch --show-current)" != "$(SYNC_BRANCH)" ]; then \
		echo "Checkout $(SYNC_BRANCH) first or override SYNC_BRANCH=..."; \
		exit 1; \
	fi
	git merge "$(UPSTREAM_REMOTE)/$(UPSTREAM_BRANCH)"

sync-merge-release:
	@test -n "$(RELEASE_TAG)" || (echo "Usage: make sync-merge-release RELEASE_TAG=v0.1.7" && exit 1)
	@if [ "$$(git branch --show-current)" != "$(RELEASE_BRANCH)" ]; then \
		echo "Checkout $(RELEASE_BRANCH) first or override RELEASE_BRANCH=..."; \
		exit 1; \
	fi
	@git rev-parse "$(RELEASE_TAG)^{tag}" >/dev/null 2>&1 || git rev-parse "$(RELEASE_TAG)^{commit}" >/dev/null 2>&1 || \
		(echo "Release tag $(RELEASE_TAG) is not available locally. Run make sync-fetch first." && exit 1)
	git merge "$(RELEASE_TAG)"

sync-validate:
	./scripts/ci/docs_quality_gate.sh
	./scripts/ci/docs_links_gate.sh
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --locked --all-targets -- -D warnings
	$(CARGO) test --locked --verbose
