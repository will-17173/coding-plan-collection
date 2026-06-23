# Repository Guidelines

## Project Structure & Module Organization

This repository is a compact reference collection for AI coding and token plans.

- `README.md` is the primary Chinese-language comparison page and source of truth for domestic Coding Plan, Token Plan, Membership Plan, and Agent Plan tables.
- `ide-plans.md` contains international AI IDE subscription comparisons.
- `api_proxy.rs` is a standalone Rust proxy implementation related to AtomCode/GitCode OpenAI-compatible endpoints.
- There are no dedicated `src/`, `tests/`, or asset directories. Add root files only for peer reference documents; otherwise create a clearly named subdirectory.

## Build, Test, and Development Commands

There is no repository-wide package manager or build script.

- `rg "provider name" README.md ide-plans.md` searches existing plan entries before adding duplicates.
- `markdownlint README.md ide-plans.md AGENTS.md` checks Markdown style if `markdownlint` is installed.
- `rustfmt api_proxy.rs` formats the Rust proxy file.
- `rustc --edition 2021 api_proxy.rs` is not expected to pass alone because it depends on external crates and missing project context.

## Coding Style & Naming Conventions

Keep Markdown tables consistent with existing columns: `套餐`, `价格`, `额度`, `支持模型`, and `备注` where applicable. Use Chinese section headings, preserve official product names, and include official links after each vendor section.

For Rust edits, follow standard Rust style: 4-space indentation, `snake_case` functions, `SCREAMING_SNAKE_CASE` constants, and run `rustfmt` before committing.

## Testing Guidelines

For documentation changes, verify table rendering in a Markdown preview and check internal links, especially `README.md` links to `ide-plans.md`. Confirm new pricing, model lists, and dates against official sources.

For `api_proxy.rs`, add or update tests in the owning Rust project if this file is copied back into a larger codebase. In this standalone repository, limit changes to clearly reviewable patches.

## Commit & Pull Request Guidelines

Recent commits use short imperative summaries in either English or Chinese, such as `update coding plan listings` or `添加最新动态：...`. Keep commit titles brief and specific.

Pull requests should include:

- A summary of changed providers, plans, or proxy behavior.
- Source links for pricing/model updates.
- Screenshots or rendered Markdown previews when table structure changes.
- Any caveats about uncertain availability, limited-time offers, or region-specific access.

## Agent-Specific Instructions

Do not invoke optional skills or automation unless requested. Keep edits scoped, preserve existing user changes, and avoid broad rewrites of plan tables unless the task explicitly asks for a refresh.
