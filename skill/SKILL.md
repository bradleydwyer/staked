---
name: pkg-check
description: "Check package name availability across registries (npm, PyPI, crates.io, RubyGems, NuGet, Hex, pub.dev, Homebrew, Docker Hub, CocoaPods, and 19 more). Use when a user wants to find an available name for a library, tool, or package, or is researching whether a name is taken."
allowed-tools:
  - Bash(pkg-check:*)
user-invocable: true
argument-hint: "<name> [name2 name3 ...]"
metadata:
  author: bradleydwyer
  version: "0.1.0"
  status: experimental
---

# pkg-check -- Package Name Availability Checker

Checks whether a package name is available across 29 registries. Always use `-j` for JSON output.

## When to Use This Skill

- User wants to know if a package name is taken
- Brainstorming or comparing names for a new library/tool
- Checking a name across specific registries or languages

## Installation

The `pkg-check` CLI must be available on PATH. Verify with `pkg-check --list-registries`. Install before proceeding if not found.

## Workflow

### Step 1: Determine Scope

| User Says | Flags |
|---|---|
| "is X available?" (general) | (none -- checks 10 popular registries) |
| "check all registries" | `-a` (all 29) |
| "check on npm and crates" | `-r npm,crates` |
| "I'm writing a Rust crate" | `-l rust` |
| multiple candidate names | pass all names as positional args |

### Step 2: Run the Check

Always use `-j` for JSON output:

```bash
pkg-check -j myname                    # 10 popular registries
pkg-check -j foo bar baz               # multiple candidates
pkg-check -j -a myname                 # all 29 registries
pkg-check -j -r npm,crates,pypi myname # specific registries
pkg-check -j -l rust,python myname     # filter by language
```

### Step 3: Report Results

Parse the JSON and present a clear summary:
- Lead with the verdict: is the name available where it matters?
- Group results by available/taken
- If checking multiple names, compare them side by side
- Call out conflicts on high-priority registries (npm, PyPI, crates.io) explicitly
- When choosing between candidates, recommend the name with the broadest availability

## CLI Quick Reference

```bash
pkg-check myname                # 10 popular registries
pkg-check foo bar baz           # multiple names
pkg-check -j myname             # JSON output (always use this)
pkg-check -v myname             # verbose per-registry detail
pkg-check -a myname             # all 29 registries
pkg-check -r npm,crates myname  # specific registries
pkg-check -l rust,python myname # filter by language
pkg-check --list-registries     # show all supported registries
pkg-check mcp                   # start MCP server
```

## Tips

- Always use `-j`. Human-readable output is for direct terminal use only.
- When a user says "Rust" or "Python", use `-l` rather than listing individual registries.
- If a name is taken, suggest variations and check those too.
