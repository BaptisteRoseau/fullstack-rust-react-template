/**
 * Scaffolds a page-shell layout under `src/components/layouts/` and re-exports it
 * from the barrel. Flesh out following `.claude/skills/frontend-react-layout`
 * (the `container` class does the centering — don't re-add `mx-auto max-w-*`).
 *
 * @type {import('plop').PlopGenerator}
 */
module.exports = {
    description: 'Layout generator (src/components/layouts)',
    prompts: [
        {
            type: 'input',
            name: 'name',
            message: 'layout name without "Layout" suffix (e.g. "settings")',
        },
    ],
    actions: () => [
        {
            type: 'add',
            path: 'src/components/layouts/{{kebabCase name}}-layout.tsx',
            templateFile: 'generators/layout/layout.tsx.hbs',
        },
        {
            type: 'append',
            path: 'src/components/layouts/index.ts',
            pattern: /export \* from '\.\/content-layout'/,
            template: "export * from './{{kebabCase name}}-layout'",
        },
    ],
}
