# Fingerprint-First Development Rules (2026-04-03)

## Purpose

These rules define the default development priority for this project under the current runtime environment:
- Ubuntu server
- headless execution
- limited CPU / memory / disk budget
- proxy-required network access

The guiding rule is:

> **Fingerprint realism has higher priority than screenshot/UI richness.**

## Core rule

When a feature tradeoff appears, default to the path that improves:
- real fingerprint consumption
- fingerprint/network/region consistency
- diagnosability of fingerprint behavior
- performance efficiency without collapsing usable concurrency

Do **not** default to screenshot, GUI, or other visually attractive but lower-value features.

## Mandatory priority order

### P0: Must be prioritized first
1. fingerprint field priority layering
2. real fingerprint consumption in the runner
3. fingerprint consumption visibility
4. fingerprint/profile/strategy/task binding
5. fingerprint regression tests

### P1: Must follow immediately
6. fingerprint-proxy-region consistency checks
7. fingerprint performance budget system
8. concurrency budget rules that preserve usable throughput

### P2: Only keep browser actions that support fingerprint validation
9. get_html
10. get_title
11. get_final_url
12. extract_text
13. extract_meta

These actions exist to support fingerprint realism and low-cost validation, not to maximize feature count.

## Explicitly de-prioritized / frozen for current stage

The following are **not** default-priority features for the current environment:
- screenshot
- visual diff / visual regression
- GUI / desktop interaction automation
- heavy binary artifact workflows
- broad script-execution expansion
- feature expansion that increases resource cost but does not improve fingerprint realism

## Fingerprint field layering

### L1: Must be truly consumed first
- user_agent
- locale / accept_language
- timezone
- viewport
- platform

### L2: High-value next layer
- client hints
- hardware_concurrency
- device_memory
- color_scheme

### L3: Advanced fingerprint layer
- canvas
- webgl
- audio
- fonts
- anti-detection flags

## Consistency rules

The system should explicitly check consistency between:
- target_region
- proxy_region
- exit_region
- timezone
- locale
- accept_language
- fingerprint profile
- network policy

A feature that improves this consistency is higher-value than a feature that only adds visual/browser richness.

## Performance rules

Performance optimization must follow this rule:

> **Optimize aggressively, but never by collapsing the system into pseudo-serial execution.**

That means:
- do not optimize by forcing effective single-thread behavior for multi-task workloads
- do not let 10 tasks behave like one long queue if the machine can still sustain safe parallelism
- separate light / medium / heavy fingerprint budgets
- default to light / medium under concurrency
- only use heavy fingerprint paths under explicit constraints

## Resource-aware development rule

Given the current host constraints, prefer:
- structured text results over binary artifacts
- previews over full dumps
- low-cost observability over high-cost visual capture
- bounded action expansion over wide browser-action expansion

## Decision rule for future development

When choosing between two candidate features, prefer the one that more directly improves:
1. fingerprint realism
2. fingerprint consumption truthfulness
3. region / identity consistency
4. throughput-preserving performance
5. diagnosability

If a feature does not materially improve any of the above, it should be delayed.
