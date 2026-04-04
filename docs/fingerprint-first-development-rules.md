# Fingerprint-First Development Rules (2026-04-03)

## Purpose

These rules define the default development priority for this project under the current runtime environment and the user's stated priorities.

Current assumptions:
- Ubuntu server runtime
- headless execution by default
- no desktop / GUI session as the normal operating mode
- proxy-required external access
- fingerprint realism has higher priority than visual/browser richness
- performance must be optimized aggressively **without degrading the system into pseudo-serial execution**
- disk capacity is expected to expand significantly (for example to ~180G), so disk is **not** the primary limiting factor anymore

The guiding rule is:

> **Absolute fingerprint priority comes first.**

That means when tradeoffs appear, default to the path that most directly improves:
1. real fingerprint consumption
2. fingerprint / proxy / region consistency
3. throughput-preserving performance
4. diagnosability
5. only then richer action surface or heavier artifacts

---

## Core rule

When a feature tradeoff appears, prefer the feature that improves:
- real fingerprint consumption in the runner
- truthful visibility of what fingerprint fields are actually consumed
- consistency between fingerprint, proxy, target region, and observed exit region
- performance efficiency that still preserves usable concurrency
- structured diagnosability via result JSON / summary artifacts / explain outputs

Do **not** default to screenshot, GUI, or other visually attractive but lower-value features.

---

## Mandatory priority order

### P0: Must be prioritized first
1. fingerprint field priority layering
2. real fingerprint consumption in the runner
3. fingerprint consumption visibility
4. fingerprint profile / strategy / task binding
5. fingerprint regression tests

### P1: Must follow immediately
6. fingerprint-proxy-region consistency checks
7. fingerprint performance budget system
8. concurrency budget rules that preserve usable throughput

### P2: Browser actions only when they support fingerprint validation or low-cost structured automation
9. get_html
10. get_title
11. get_final_url
12. extract_text
13. extract_links
14. extract_meta
15. response_meta / response_headers / page_meta

These are also the strongest candidates for the first browser-facing API entry surface above the task layer.

These actions exist to support fingerprint realism, low-cost validation, and structured automation — **not** to maximize feature count for its own sake.

---

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

### Layering rule

A new L2/L3 field should not delay unfinished L1 real-consumption work.
A feature that makes L1 more truthful is usually more valuable than a feature that only broadens action count.

---

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

The output should be able to express states like:
- exact_match
- soft_match
- mismatch
- missing_context
- suspicious_combination

A feature that improves this consistency is higher-value than a feature that only adds visual/browser richness.

---

## Performance rules

Performance optimization must follow this rule:

> **Optimize aggressively, but never by collapsing the system into pseudo-serial execution.**

This means:
- do not optimize by effectively forcing single-thread behavior for multi-task workloads
- do not let 10 tasks behave like 1 slow queue if the machine can safely sustain parallel work
- separate light / medium / heavy fingerprint budgets
- default to light / medium under concurrency
- only use heavy fingerprint paths under explicit constraints
- preserve usable throughput as a first-class requirement

### Concurrency rule

Performance wins are invalid if they make the system feel artificially serialized.
The system should stay meaningfully concurrent under safe load instead of becoming “technically stable but practically stalled.”

### Budget rule

Fingerprint features must be designed with:
- light budget
- medium budget
- heavy budget

Heavy paths must not silently become the default for all tasks.

---

## Runtime-environment rule

Current environment reality matters.
This project should be developed for a **headless Ubuntu server** first, not for a desktop-first environment.

## Product entry rule

The final operation entry for the fingerprint browser should be an API surface.

That means:
- external callers should be able to treat the system as a browser API product
- the task queue/control plane may remain underneath, but should not be the final product mental model
- new browser capabilities should be evaluated partly by whether they help shape a clean browser-facing API

That means the default preference is:
- text / JSON / structured outputs
- low-cost page-read style actions
- network diagnostics
- explainability
- runner truthfulness
- proxy / region / identity consistency

Instead of:
- desktop interaction
- visual flows
- GUI assumptions
- screenshot-first thinking

---

## Explicitly de-prioritized / frozen for the current stage

The following are **not** default-priority features for the current environment and current project goal:
- screenshot as a mainline feature
- visual diff / visual regression as a mainline feature
- GUI / desktop interaction automation
- desktop-window-dependent browser actions
- broad script-execution expansion
- feature expansion that increases resource cost but does not improve fingerprint realism

### Important nuance

Disk capacity is no longer treated as the main blocker.
So these features are not frozen **because disk is too small**.
They are frozen or de-prioritized because they are lower-value under:
- fingerprint-first priorities
- headless Ubuntu runtime
- current project mainline value ordering

In other words:
- screenshot is **not** the right next feature even if disk becomes large
- GUI/desktop interaction is still a poor match for the runtime
- broad visual workflows still distract from fingerprint realism and structured automation

---

## What becomes more acceptable after disk expansion

Because disk is expected to expand significantly, the following can move from “strongly constrained” to “allowed as later enhancements”:
- HTML full retention (when useful)
- richer debug artifacts
- network dumps
- runner traces
- structured debug retention
- more complete result/debug artifact preservation

But these are still **secondary** to fingerprint realism and throughput-preserving performance.

### Disk rule

More disk means more room for:
- better retention policies
- richer debug artifacts
- fuller structured traces

It does **not** automatically make screenshot / GUI features a good priority choice.

---

## Resource-aware development rule

Given the current host/runtime constraints, prefer:
- structured text results over binary artifacts
- previews over full dumps by default
- low-cost observability over high-cost visual capture
- bounded action expansion over wide browser-action expansion
- structured explain outputs over image-heavy validation

### Preferred artifact hierarchy

Prefer this order by default:
1. JSON result fields
2. text preview / html preview / meta preview
3. structured explain / summary artifact
4. optional full HTML / trace / dump retention
5. screenshot / other heavy binary artifacts only when a later stage explicitly justifies them

---

## Browser-action decision rule

A new browser action is justified only if it materially helps one of these:
- fingerprint realism validation
- structured page understanding
- network / redirect / page-state diagnosis
- low-cost automation output

That is why the current preferred next-action family is:
- get_title
- get_final_url
- get_html
- extract_text
- extract_links
- extract_meta

And not:
- screenshot-first expansion
- GUI-first expansion
- broad script execution without tight boundaries

---

## Decision rule for future development

When choosing between two candidate features, prefer the one that more directly improves:
1. fingerprint realism
2. fingerprint consumption truthfulness
3. region / identity consistency
4. throughput-preserving performance
5. diagnosability
6. structured automation usefulness

If a feature does not materially improve any of the above, it should be delayed.

---

## One-sentence summary

> **This project should evolve as a headless, fingerprint-first, proxy-aware, structured-automation system — not as a GUI-first or screenshot-first browser toy.**
