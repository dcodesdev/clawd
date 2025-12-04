# Clawd Skills

Open-source collection of Claude skills.

## Structure

```txt
skills/
└── <author>/
    └── <skill-name>/
        ├── README.mdx      # Documentation with frontmatter metadata
        └── skill/          # Skill content (gets packaged)
            ├── SKILL.md    # Main skill file
            └── ...         # Additional files
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## Schema Validation

Skills are validated against:

- `readme-schema.json` - README.mdx frontmatter
- `skill-schema.json` - SKILL.md frontmatter

Run validation:

```bash
pnpm test
```
