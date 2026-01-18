# CLAUDE.md — Project Context for AI Agents

> This file provides context for AI coding agents (Claude Code, Cursor, etc.) working on this codebase.

## ⚠️ CRITICAL: Task Tracking Rules

This project uses **bd (beads)** for issue tracking.
Run `bd prime` for workflow context, or install hooks (`bd hooks install`) for auto-injection.

**Quick reference:**
- `bd ready` - Find unblocked work
- `bd create "Title" --type task --priority 2` - Create issue
- `bd close <id>` - Complete work
- `bd sync` - Sync with git (run at session end)

For full workflow details: `bd prime`

## Project Overview

> **Agnx is the "nginx for AI agents"** — a minimal, fast, self-hostable runtime that runs agents defined in a **transparent, portable format**, exposed through a standard API.

Agnx treats agents as durable artifacts: files you own that should outlast the runtime.
- **Transparent agent format** (human-readable, inspectable, versionable)
- **Stateless by default** (no hidden server-side state)
- **File-based state** when present (specs, memories, logs, config) — if Agnx disappears, take these and host elsewhere

## Strategic Documents

- **[Readme of this project](README.md)**
- **[Project status / roadmap](./docs/specs/PROJECT_STATUS.md)**
- **[Project Charter](./docs/specs/202601111100.project-charter.md)**
- **[Architecture](./docs/specs/202601111101.architecture.md)**
- **[API Reference](./docs/specs/202601111102.api-reference.md)**
- **[Deployment](./docs/specs/202601111103.deployment.md)**
- **[Agnx Agent Format (AAF)](./docs/specs/202601111200.agnx-agent-format.md)**
- **[Example skill](./docs/examples/skills/task-extraction/)**

## Tech Stack

- Rust 2024 Edition (single binary, <5MB, no runtime dependencies)
- HTTP API: Axum + SSE
- Config/spec: YAML + Markdown
- Tool ecosystem: MCP
- Discovery: A2A Agent Card
- Targets: x86_64, ARM64, ARM32 (edge/embedded-capable)
