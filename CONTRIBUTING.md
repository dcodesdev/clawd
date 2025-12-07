# Contributing

## Adding a Skill

1. Create directory: `skills/<your-author-id>/<skill-name>/`
2. Add yourself to `authors.json` if not present
3. Create `README.mdx` with required frontmatter
4. Run `pnpm test` to validate
5. Submit PR

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
---
```

## Categories

- Development
- DevOps
- Writing
- QA
- Security
- Data
- Design
- Other
