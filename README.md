# Clawd

[![GitHub stars](https://img.shields.io/github/stars/dcodesdev/clawd?style=social)](https://github.com/dcodesdev/clawd)

Open-source collection of Claude skills. Clawd will install the Claude skills you want for you.

## Installation

```bash
curl -fsSL https://api.clawd.xyz/install.sh | sh
```

Or install a specific version:

```bash
curl -fsSL https://api.clawd.xyz/install.sh | sh -s v1.0.0
```

## Usage

```bash
# List all available skills
clawd list

# Search for skills
clawd search brainstorming

# Add a skill
clawd add obra/brainstorming

# Update to latest version
clawd upgrade
```

## Repository Structure

```text
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
│           ├── README.mdx  # Metadata + documentation
│           └── skill/      # Skill content (packaged for users)
│               ├── SKILL.md
│               └── ...     # Additional skill files
├── authors.json            # Author registry
├── readme-schema.json      # README.mdx validation schema
├── skill-schema.json       # SKILL.md validation schema
└── install.sh              # curl | sh installer script
```

## Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

### Quick Start

1. Fork the repository
2. Add yourself to `authors.json` if you're a new contributor
3. Create your skill directory: `skills/<your-id>/<skill-name>/`
4. Add `README.mdx` with frontmatter (see schema)
5. Add `skill/SKILL.md` with frontmatter
6. Run `pnpm test` to validate
7. Submit a pull request

### Frontmatter Schemas

#### README.mdx

Required fields:

- `id` - Format: `author/skill-name`
- `title` - Display name
- `description` - Brief description
- `version` - Semantic version (e.g., 1.0.0)
- `category` - One of: Development, DevOps, Writing, QA, Security, Data, Design, Other
- `author` - Your author ID from authors.json
- `repo` - GitHub repository URL
- `path` - Path to skill directory

#### skill/SKILL.md

Required fields:

- `name` - Skill name (kebab-case)
- `description` - When and how to use this skill

See `readme-schema.json` and `skill-schema.json` for complete specifications.

## Development

### Prerequisites

- Rust (for CLI development)
- Node.js + pnpm (for validation)

### CLI Development

```bash
cd cli
cargo build                 # Debug build
cargo build --release       # Release build
cargo run -- list           # Run with arguments
cargo test                  # Run tests
```

### Validation

```bash
pnpm install
pnpm test                   # Validate all skills against schemas
pnpm test:watch             # Watch mode
```

## Environment Variables

- `CLAWD_API_URL` - Override backend API URL (default: production)
- `CLAWD_INSTALL_DIR` - Override install directory (default: /usr/local/bin)

## License

MIT License - see [LICENSE](./LICENSE) for details.

## Authors

See [authors.json](./authors.json) for the list of contributors.
