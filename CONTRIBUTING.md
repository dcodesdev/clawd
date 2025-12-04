# Contributing

## Adding a Skill

1. Create directory: `skills/<your-author-id>/<skill-name>/`
2. Add yourself to `authors.json` if not present
3. Create `README.mdx` with required frontmatter
4. Create `skill/SKILL.md` with name/description frontmatter
5. Add any additional files in `skill/`
6. Run `pnpm test` to validate
7. Submit PR

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

### skill/SKILL.md

```yaml
---
name: skill-name
description: When and how to use this skill
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
