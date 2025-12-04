import { describe, it, expect } from 'vitest'
import Ajv from 'ajv'
import addFormats from 'ajv-formats'
import matter from 'gray-matter'
import fs from 'fs'
import path from 'path'

const ajv = new Ajv({ strict: false })
addFormats(ajv)

const rootDir = path.resolve(__dirname, '..')
const readmeSchema = JSON.parse(
  fs.readFileSync(path.join(rootDir, 'readme-schema.json'), 'utf-8'),
)
const skillSchema = JSON.parse(
  fs.readFileSync(path.join(rootDir, 'skill-schema.json'), 'utf-8'),
)
const authors = JSON.parse(
  fs.readFileSync(path.join(rootDir, 'authors.json'), 'utf-8'),
)

const validateReadme = ajv.compile(readmeSchema)
const validateSkill = ajv.compile(skillSchema)

function getSkillDirs(): string[] {
  const skillsDir = path.join(rootDir, 'skills')
  if (!fs.existsSync(skillsDir)) return []

  const dirs: string[] = []
  const authorDirs = fs
    .readdirSync(skillsDir, { withFileTypes: true })
    .filter((d) => d.isDirectory())

  for (const authorDir of authorDirs) {
    const authorPath = path.join(skillsDir, authorDir.name)
    const skillDirs = fs
      .readdirSync(authorPath, { withFileTypes: true })
      .filter((d) => d.isDirectory())

    for (const skillDir of skillDirs) {
      dirs.push(path.join(authorPath, skillDir.name))
    }
  }

  return dirs
}

describe('skill validation', () => {
  const skillDirs = getSkillDirs()

  it('should have at least one skill', () => {
    expect(skillDirs.length).toBeGreaterThan(0)
  })

  for (const skillDir of skillDirs) {
    const skillId = skillDir.split('/skills/')[1]

    describe(skillId, () => {
      it('should have README.mdx', () => {
        const readmePath = path.join(skillDir, 'README.mdx')
        expect(fs.existsSync(readmePath)).toBe(true)
      })

      it('should have valid README.mdx frontmatter', () => {
        const readmePath = path.join(skillDir, 'README.mdx')
        const content = fs.readFileSync(readmePath, 'utf-8')
        const { data } = matter(content)

        const valid = validateReadme(data)
        if (!valid) {
          console.error('README.mdx validation errors:', validateReadme.errors)
        }
        expect(valid).toBe(true)
      })

      it('should have matching skill ID and directory structure', () => {
        const readmePath = path.join(skillDir, 'README.mdx')
        const content = fs.readFileSync(readmePath, 'utf-8')
        const { data } = matter(content)

        expect(data.id).toBe(skillId)
      })

      it('should have author in authors.json', () => {
        const readmePath = path.join(skillDir, 'README.mdx')
        const content = fs.readFileSync(readmePath, 'utf-8')
        const { data } = matter(content)

        expect(authors[data.author]).toBeDefined()
      })

      it('should have contributors in authors.json', () => {
        const readmePath = path.join(skillDir, 'README.mdx')
        const content = fs.readFileSync(readmePath, 'utf-8')
        const { data } = matter(content)

        if (data.contributors) {
          for (const contributor of data.contributors) {
            expect(authors[contributor]).toBeDefined()
          }
        }
      })

      it('should have skill/ directory', () => {
        const skillPath = path.join(skillDir, 'skill')
        expect(fs.existsSync(skillPath)).toBe(true)
        expect(fs.statSync(skillPath).isDirectory()).toBe(true)
      })

      it('should have skill/SKILL.md', () => {
        const skillMdPath = path.join(skillDir, 'skill', 'SKILL.md')
        expect(fs.existsSync(skillMdPath)).toBe(true)
      })

      it('should have valid skill/SKILL.md frontmatter', () => {
        const skillMdPath = path.join(skillDir, 'skill', 'SKILL.md')
        const content = fs.readFileSync(skillMdPath, 'utf-8')
        const { data } = matter(content)

        const valid = validateSkill(data)
        if (!valid) {
          console.error('SKILL.md validation errors:', validateSkill.errors)
        }
        expect(valid).toBe(true)
      })
    })
  }
})
