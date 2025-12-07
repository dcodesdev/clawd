# Contributing

## Adding a Skill

1. Create directory: `skills/<your-author-id>/<skill-name>/`
2. Add yourself to `authors.json` if not present
3. Create `README.mdx` with required frontmatter
4. Run `pnpm test` to validate your changes
5. Submit PR

### Validation

Before submitting, run `pnpm test` to ensure your skill passes validation:

```bash
pnpm install  # If you haven't installed dependencies
pnpm test     # Run validation tests
```

The validation checks:

- `README.mdx` exists and has valid frontmatter
- All required fields are present and correctly formatted
- Skill ID matches the directory structure (`<author>/<skill-name>`)
- Author exists in `authors.json`
- Contributors (if specified) exist in `authors.json`
- Version follows semantic versioning (e.g., `1.0.0`)

Fix any validation errors before submitting your PR.

## Frontmatter Requirements

### README.mdx

```yaml
---
id: author/skill-name
title: Skill Title
description: Brief description
version: 1.0.0
category: Development
author: your-author-id
repo: your-handle/your-repo
path: skills/your-handle/skill-name
tags:
  - tag1
  - tag2
---
```

**Required fields:**

- `id` - Format: `<author>/<skill-name>` (must match directory structure)
- `title` - Human-readable skill title
- `description` - Brief description of the skill
- `version` - Semantic version (e.g., `1.0.0`)
- `category` - One of the allowed categories (see below)
- `author` - Author ID from `authors.json`
- `repo` - GitHub repository in `owner/repo` format
- `path` - Directory path within the repository

**Optional fields:** `tags`, `contributors`, `created_at`, `updated_at`, `license`, `requirements`, `ref`, `isVerified`

**Field formats:**

- `id`: Must match pattern `^[a-z0-9-]+/[a-z0-9-]+$`
- `version`: Must match semantic versioning `^\\d+\\.\\d+\\.\\d+$`
- `author`: Must match pattern `^[a-z0-9-]+$` and exist in `authors.json`
- `repo`: Must match pattern `^[a-zA-Z0-9_-]+/[a-zA-Z0-9_.-]+$`

## Categories

- Development
- DevOps
- Writing
- QA
- Security
- Data
- Design
- Other
