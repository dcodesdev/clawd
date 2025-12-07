# CLAUDE.md

## About

Clawd is an open-source collection of Claude skills with a CLI for browsing and installing them.

## Repository Structure

```
clawd/
├── cli/                    # Rust CLI application
│   ├── src/
│   │   ├── main.rs         # Entry point, command definitions
│   │   ├── api/            # API clients (GitHub, Clawd backend)
│   │   ├── config.rs       # Configuration handling
│   │   ├── download.rs     # Skill download logic
│   │   ├── list.rs         # List skills command
│   │   ├── upgrade.rs      # Self-update command
│   │   └── prompts.rs      # Interactive prompts
│   └── Cargo.toml
├── skills/                 # Skill definitions
│   └── <author>/
│       └── <skill-name>/
│           └── README.mdx  # Metadata + documentation
├── authors.json            # Author registry
├── readme-schema.json      # README.mdx validation schema
└── install.sh              # curl | sh installer script
```

## CLI Commands

```bash
clawd list                  # List available skills
clawd search <query>        # Search for skills
clawd add <id>               # Add a skill (e.g., dcodes/brainstorming)
clawd upgrade               # Self-update to latest version
```

## Development

### CLI (Rust)

```bash
cd cli
cargo build                 # Debug build
cargo build --release       # Release build
cargo run -- list           # Run with arguments
```

### Validation

```bash
pnpm test                   # Validate all skills against schemas
```

## Adding Skills

1. Add yourself to `authors.json` if new
2. Create `skills/<author>/<skill-name>/`
3. Add `README.mdx` with frontmatter (see `readme-schema.json`)
4. Run `pnpm test` to validate
5. Submit PR

## Schemas

### README.mdx Frontmatter

Required fields: `id`, `title`, `description`, `version`, `category`, `author`, `repo`, `path`

Categories: `Development`, `DevOps`, `Writing`, `QA`, `Security`, `Data`, `Design`, `Other`

## Environment Variables

- `CLAWD_API_URL` - Override backend API URL (default: production)
