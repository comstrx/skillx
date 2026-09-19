# ✨ SkillX

<div align="center">
      <img height="350" src="https://github.com/user-attachments/assets/c133029e-dabb-4919-b8a4-f96aa6f2e5d5" />
</div>

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/comstrx/skillx/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/comstrx/skillx/actions/workflows/ci.yaml)
[![Release](https://img.shields.io/github/v/release/comstrx/skillx?sort=semver)](https://github.com/comstrx/skillx/releases/latest)

## Overview

`SkillX` is an engineering intelligence layer for AI coding agents.

It provides reusable engineering knowledge, stack composition, project workflows, and execution constraints for building production software consistently across multiple AI models and agents.

SkillX does not replace the agent.

It gives the agent a stronger engineering system to operate within.

---

## Why SkillX

Modern AI models can generate large amounts of code.

The harder problem is ensuring that code follows the same standards across an entire product:

* architecture
* security
* reliability
* performance
* testing
* observability
* data integrity
* infrastructure awareness
* UI/UX quality
* business rules
* review and evaluation

SkillX centralizes that knowledge and makes it reusable across projects.

The model handles execution.

SkillX defines the engineering constraints, knowledge, and workflows around that execution.

---

## Architecture

SkillX knowledge is divided into three layers:

```text
seed/
├── foundation/
│   ├── business/
│   ├── capability/
│   ├── experience/
│   ├── language/
│   ├── library/
│   └── stack/
│
└── workflow/
```

### Foundation

Reusable engineering knowledge.

```text
business/    Domain and product knowledge
capability/  Databases, brokers, search, platforms, integrations
experience/  Engineering practices for API, infra, web, panel, and mobile
language/    Language-specific engineering rules
library/     Framework and library knowledge
stack/       Dependency composition for complete technology stacks
```

### Workflow

Defines how an AI agent should operate during the engineering lifecycle.

```text
start
learn
plan
code
compose
format
test
debug
review
audit
leak
bench
refactor
reinforce
eval
suggest
verify
visual
doc
push
```

---

## Stack Composition

Stacks combine reusable knowledge from the foundation tree.

Initial stacks include:

```text
api-actix
api-laravel
infra-bash
lib-rust
mobile-expo
panel-next
web-next
```

A stack can compose knowledge from multiple sources.

Example:

```text
api-actix
├── language/rust
├── library/actix
├── capability/data/postgresql
├── capability/data/redis
├── experience/api/architecture
├── experience/api/security
├── experience/api/transactions
├── experience/api/observability
└── ...
```

The stack definition describes what knowledge must be loaded.

The files themselves contain the engineering knowledge.

---

## Project Model

A complete product is expected to live in a single repository:

```text
product/
├── specs/
├── server/
├── panel/
├── web/
├── mobile/
└── infra/
```

`specs/` contains the product-specific knowledge:

* business rules
* workflows
* roles
* permissions
* entities
* states
* policies
* integrations
* product requirements

SkillX provides reusable engineering intelligence.

The project repository provides product-specific intelligence.

Together they give the agent the context required to build the system.

---

## AI Agent Integration

SkillX is model-independent.

The runtime is designed to expose SkillX through:

```text
CLI
MCP
```

This allows compatible agents such as:

```text
Claude Code
Codex
AgentX
Future coding agents
```

to request stack knowledge, workflows, and project context without embedding SkillX into the agent itself.

```text
AI Agent
    ↓
CLI / MCP
    ↓
SkillX
    ↓
Foundation + Stack + Workflow + Project Specs
```

The AI model remains replaceable.

The engineering knowledge remains persistent.

---

## Engineering Lifecycle

A project can be driven through repeatable engineering passes:

```text
Requirements
    ↓
Learn
    ↓
Plan
    ↓
Code
    ↓
Test
    ↓
Review
    ↓
Audit
    ↓
Benchmark
    ↓
Reinforce
    ↓
Evaluate
    ↓
Verify
```

Each pass has a different responsibility.

For example:

* `test` validates behavior.
* `review` validates engineering quality.
* `audit` searches for deeper risks and violations.
* `bench` validates measurable performance.
* `reinforce` searches for drift from known engineering rules.
* `eval` evaluates overall project readiness.
* `verify` confirms that required conditions are actually satisfied.

---

## Compounding Engineering Knowledge

SkillX is designed to improve through real project experience.

```text
Project
   ↓
Failure / Lesson
   ↓
Engineering Knowledge
   ↓
SkillX
   ↓
Next Project
```

Reusable lessons can be extracted from production work and incorporated into the permanent knowledge base.

The result is a system where future projects can inherit engineering knowledge accumulated from previous projects.

---

## Business Value

Traditional software delivery scales heavily with engineering headcount.

SkillX is designed to increase the amount of reliable software that can be produced by a small number of engineers operating AI agents.

Its objective is to improve:

* delivery speed
* consistency
* engineering quality
* repeatability
* automation
* review depth
* reuse of engineering knowledge
* parallel project execution

The long-term target is to make complete production systems achievable in weeks rather than months when the product requirements, stack knowledge, automation, and AI execution are sufficiently mature.

SkillX does not assume that generated code is correct.

Its value comes from forcing generated software through repeatable engineering knowledge and verification workflows.

---

## Technology Direction

The initial ecosystem focuses primarily on:

```text
Rust
Bash
TypeScript
JavaScript
Node.js
PHP
```

with specialized stacks built around:

```text
Actix Web
Laravel
Next.js
Expo
```

The long-term direction favors a narrow set of deeply trained stacks rather than shallow support for every possible technology.

---

## Runtime Philosophy

The runtime should remain small.

Its responsibility is limited to:

```text
resolve knowledge
compose dependencies
load workflows
load project context
expose CLI
expose MCP
```

Engineering intelligence belongs in `seed/`.

Product intelligence belongs in project `specs/`.

The runtime only connects them.

---

## Goal

```text
Specification
    ↓
SkillX
    ↓
AI Engineering Agents
    ↓
Production Candidate
```

The objective is not to generate more code.

The objective is to produce better software with less repeated human engineering effort.

---

## Ecosystem

`skillx` is part of the ToolX ecosystem maintained by [comstrx](https://github.com/comstrx).

## Community

* [Issues](https://github.com/comstrx/skillx/issues)
* [Discussions](https://github.com/comstrx/skillx/discussions)
* [Contributing](https://github.com/comstrx/skillx/blob/main/CONTRIBUTING.md)
* [Security](https://github.com/comstrx/skillx/blob/main/SECURITY.md)
* [Support](https://github.com/comstrx/skillx/blob/main/SUPPORT.md)

## License

`skillx` is dual-licensed under either:

* [MIT](https://github.com/comstrx/skillx/blob/main/LICENSE-MIT)
* [Apache-2.0](https://github.com/comstrx/skillx/blob/main/LICENSE-APACHE)

at your option.

Unless explicitly stated otherwise, contributions intentionally submitted for inclusion in this project, as defined by the Apache-2.0 license, are dual-licensed under the same terms.
