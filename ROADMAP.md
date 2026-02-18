# Roadmap

## v0.1.x — Polish & UX

- **Wizard interaction hints**: Show keybinding hints in MultiSelect prompts (space to toggle, enter to confirm) — `dialoguer` doesn't surface these by default
- **Clarify plugin cache source**: Make it clear that `~/.claude/plugins/cache` refers to *active* plugins installed from the Claude Code marketplace, not arbitrary cached files
- **Wizard visual polish**: Add more color, section dividers, and summary output using `console::style()` — helpful cues without clutter
- **Explain symlink model in wizard**: Clarify that the library uses symlinks (originals are never moved or copied), so users understand there's no data loss risk
- **Optional git init for library**: Ask during `skync init` whether to initialize a git repo in the library directory for change tracking across syncs

## v0.2 — Connector Architecture

The current model hardcodes targets as struct fields and keeps source/target logic separate. Both sides are really the same concept: an **endpoint with a connector type** that knows how to discover, read, write, and translate skills.

- **Generic `[[targets]]` array**: Replace the hardcoded `Targets` struct with a `Vec<Target>` — same shape as sources. Each target has a `name`, `path`, `type`, and connector-specific options
- **Connector trait**: Unified interface for both source and target behavior — discovery format, distribution method (symlink, MCP config, copy), and format translation needs
- **Built-in connectors**: Claude (plugins + standalone), Codex, Antigravity, Cursor, Windsurf, OpenCode, Nanobot, PicoClaw, OpenClaw
- **Bidirectional by design**: Any connector can act as both source and target — discover skills *from* Cursor rules and distribute *to* Cursor rules
- **Format awareness per connector**: Each connector declares its native format — the pipeline handles translation between them (e.g., SKILL.md ↔ Cursor rules ↔ Windsurf conventions)
- Support syncing `.claude/rules/` and agent definitions alongside skills

## v0.3 — Format Transforms

- Pluggable transform pipeline driven by connector format declarations
- Preserve original format — transforms are output-only
- Connectors declare input/output formats; the pipeline resolves the translation chain

## v0.4 — Git Sources

- Add `type = "git"` source for remote skill repositories
- Clone/pull on sync with caching
- Pin to branch, tag, or commit SHA
- Support private repos via SSH keys or token auth

## v0.5 — Watch Mode

- `skync watch` for auto-sync on filesystem changes
- Debounced fsnotify-based watcher
- Optional desktop notification on sync

## Future Ideas

- **Plugin registry**: Browse and install community skill packs
- **Conflict resolution UI**: Interactive merge when skills collide
- **Skill validation**: Lint SKILL.md for common issues (missing frontmatter, broken links)
- **Shell completions**: Generate completions for bash, zsh, fish
- **Homebrew formula**: `brew install skync`
- **Backup snapshots**: Optional tarball backup of library state before destructive operations
