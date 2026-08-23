# design-system

Domain-agnostic UI primitives. Nothing here may import `api/`, `contexts/`, `components/`,
`pages/` or `layouts/` — ESLint enforces it. A primitive receives everything through props; if it
needs data, that data comes from a parent in `components/` or `pages/`.

```txt
design-system/
├── <ComponentName>/
│   ├── <ComponentName>.tsx
│   ├── <ComponentName>.test.tsx
│   ├── <ComponentName>.stories.tsx     # mandatory for every primitive
│   ├── <component-name>.module.scss
│   └── index.ts
├── inputs/                              # lowercase grouping folder, no barrel of its own
│   ├── TextInput/
│   ├── TextArea/
│   ├── SelectInput/
│   ├── CheckboxInput/
│   └── SwitchInput/
└── Icon/
    ├── makeIcon.tsx                     # wraps an SVG into an accessible icon component
    └── resources/*.svg                  # raw source SVGs
```

Every component here has a Storybook story. A component that cannot be told in a story without
mocking an API belongs in `src/components/` instead. A lowercase grouping folder like `inputs/` is
allowed once a family grows past three members; import its members directly
(`@/design-system/inputs/TextInput`), since the grouping folder has no barrel.

## Skills

- [frontend-design-system](../../../.claude/skills/frontend-design-system/SKILL.md)
